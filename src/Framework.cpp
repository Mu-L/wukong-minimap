#include <chrono>
#include <filesystem>

#include <windows.h>
#include <d3d12.h>
#include <ShlObj.h>

#include <spdlog/sinks/basic_file_sink.h>

#include <imgui.h>
#include "uevr-imgui/font_robotomedium.hpp"
#include "uevr-imgui/imgui_impl_dx11.h"
#include "uevr-imgui/imgui_impl_dx12.h"
#include "uevr-imgui/imgui_impl_win32.h"

#include "utility/Module.hpp"
#include "utility/Patch.hpp"
#include "utility/Scan.hpp"
#include "utility/Thread.hpp"
#include "utility/String.hpp"
#include "utility/Input.hpp"

#include "WindowFilter.hpp"

#include "Framework.hpp"

namespace fs = std::filesystem;
using namespace std::literals;

std::unique_ptr<Framework> g_framework{};

void Framework::hook_monitor()
{
    if (!m_hook_monitor_mutex.try_lock())
    {
        // If this happens then we can assume execution is going as planned
        // so we can just reset the times so we dont break something
        m_last_present_time = std::chrono::steady_clock::now() + std::chrono::seconds(5);
        m_last_chance_time = std::chrono::steady_clock::now() + std::chrono::seconds(1);
        m_has_last_chance = true;
    }
    else
    {
        m_hook_monitor_mutex.unlock();
    }

    std::scoped_lock _{m_hook_monitor_mutex};

    if (g_framework == nullptr)
    {
        return;
    }

    const auto now = std::chrono::steady_clock::now();

    auto &d3d12 = get_d3d12_hook();

    if (d3d12 == nullptr || (d3d12 != nullptr && !d3d12->is_inside_present()))
    {
        // check if present time is more than 5 seconds ago
        if (now - m_last_present_time >= std::chrono::seconds(5))
        {
            if (m_has_last_chance)
            {
                // the purpose of this is to make sure that the game is not frozen
                // e.g. if we are debugging the game, so we don't rehook anything on accident
                m_has_last_chance = false;
                m_last_chance_time = now;

                spdlog::info("Last chance encountered for hooking");
            }

            if (!m_has_last_chance && now - m_last_chance_time > std::chrono::seconds(1))
            {
                spdlog::info("Sending rehook request for D3D");

                hook_d3d12();

                // so we don't immediately go and hook it again
                // add some additional time to it to give it some leeway
                m_last_present_time = std::chrono::steady_clock::now() + std::chrono::seconds(5);
                m_last_message_time = std::chrono::steady_clock::now() + std::chrono::seconds(5);
                m_last_chance_time = std::chrono::steady_clock::now() + std::chrono::seconds(1);
                m_has_last_chance = true;
            }
        }
        else
        {
            m_last_chance_time = std::chrono::steady_clock::now();
            m_has_last_chance = true;
        }

        if (m_initialized && m_wnd != 0 && now - m_last_message_time > std::chrono::seconds(5))
        {
            if (m_windows_message_hook != nullptr && m_windows_message_hook->is_hook_intact())
            {
                spdlog::info("Windows message hook is still intact, ignoring...");
                m_last_message_time = now;
                m_last_sendmessage_time = now;
                m_sent_message = false;
                return;
            }

            // send dummy message to window to check if our hook is still intact
            if (!m_sent_message)
            {
                spdlog::info("Sending initial message hook test");

                auto proc = (WNDPROC)GetWindowLongPtr(m_wnd, GWLP_WNDPROC);

                if (proc != nullptr)
                {
                    const auto ret = CallWindowProc(proc, m_wnd, WM_NULL, 0, 0);

                    spdlog::info("Hook test message sent");
                }

                m_last_sendmessage_time = std::chrono::steady_clock::now();
                m_sent_message = true;
            }
            else if (now - m_last_sendmessage_time > std::chrono::seconds(1))
            {
                spdlog::info("Sending reinitialization request for message hook");

                // if we don't get a message for 5 seconds, assume the hook is broken
                // m_initialized = false; // causes the hook to be re-initialized next frame
                m_message_hook_requested = true;
                m_last_message_time = std::chrono::steady_clock::now() + std::chrono::seconds(5);
                m_last_present_time = std::chrono::steady_clock::now() + std::chrono::seconds(5);

                m_sent_message = false;
            }
        }
        else
        {
            m_sent_message = false;
        }
    }
}

Framework::Framework(HMODULE framework_module)
    : m_framework_module{framework_module}, m_game_module{GetModuleHandle(0)},
      m_logger{spdlog::basic_logger_mt("UnrealVR", (get_persistent_dir() / "log.txt").string(), true)}
{
    std::scoped_lock __{m_constructor_mutex};

    spdlog::set_default_logger(m_logger);
    spdlog::flush_on(spdlog::level::info);
    spdlog::info("UnrealVR entry");

    const auto module_size = *utility::get_module_size(m_game_module);

    spdlog::info("Game Module Addr: {:x}", (uintptr_t)m_game_module);
    spdlog::info("Game Module Size: {:x}", module_size);

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();

#ifdef DEBUG
    spdlog::set_level(spdlog::level::debug);
#endif

    // Hooking D3D12 initially because we need to retrieve the command queue before the first frame then switch to D3D11 if it failed later
    // on
    // addendum: now we don't need to do that, we just grab the command queue offset from the swapchain we create
    /*if (!hook_d3d12()) {
        spdlog::error("Failed to hook D3D12 for initial test.");
    }*/

    std::scoped_lock _{m_hook_monitor_mutex};

    m_last_present_time = std::chrono::steady_clock::time_point{}; // Instantly send the first message
    m_last_message_time = std::chrono::steady_clock::time_point{}; // Instantly send the first message
    m_last_chance_time = std::chrono::steady_clock::time_point{};  // Instantly send the first message
    m_has_last_chance = false;

    m_d3d_monitor_thread = std::make_unique<std::jthread>([this](std::stop_token s)
                                                          {
        spdlog::info("D3D monitor thread entry");

        {
            std::scoped_lock _{m_constructor_mutex};

            while (g_framework == nullptr) {
                std::this_thread::yield();
            }
        }

        std::this_thread::sleep_for(std::chrono::milliseconds(33));

        while (!s.stop_requested() && !m_terminating) {
            this->hook_monitor();
            std::this_thread::sleep_for(std::chrono::milliseconds(500));
        } });

    spdlog::info("Leaving Framework constructor");
}

bool Framework::hook_d3d12()
{
    // windows 7?
    if (LoadLibraryA("d3d12.dll") == nullptr)
    {
        spdlog::info("d3d12.dll not found, user is probably running Windows 7.");
        spdlog::info("Falling back to hooking D3D11.");

        return false;
    }

    // if (m_d3d12_hook == nullptr) {
    m_d3d12_hook.reset();
    m_d3d12_hook = std::make_unique<D3D12Hook>();
    m_d3d12_hook->on_present([this](D3D12Hook &hook)
                             { on_frame_d3d12(); });
    m_d3d12_hook->on_post_present([this](D3D12Hook &hook)
                                  { on_post_present_d3d12(); });
    m_d3d12_hook->on_resize_buffers([this](D3D12Hook &hook, uint32_t w, uint32_t h)
                                    { on_reset(w, h); });
    m_d3d12_hook->on_resize_target([this](D3D12Hook &hook, uint32_t w, uint32_t h)
                                   { on_reset(w, h); });
    //}
    // m_d3d12_hook->on_create_swap_chain([this](D3D12Hook& hook) { m_d3d12.command_queue = m_d3d12_hook->get_command_queue(); });

    if (m_d3d12_hook->hook())
    {
        spdlog::info("Hooked DirectX 12");
        m_valid = true;
        return true;
    }
    // We make sure to no unhook any unwanted hooks if D3D12 didn't get hooked properly
    if (m_d3d12_hook->unhook())
    {
        spdlog::info("D3D12 Unhooked!");
    }
    else
    {
        spdlog::info("Cannot unhook D3D12, this might crash.");
    }

    m_valid = false;

    return false;
}

Framework::~Framework()
{
    spdlog::info("Framework shutting down...");

    m_terminating = true;
    m_d3d_monitor_thread->request_stop();
    if (m_d3d_monitor_thread->joinable())
    {
        m_d3d_monitor_thread->join();
    }

    deinit_d3d12();

    ImGui_ImplWin32_Shutdown();

    if (m_initialized)
    {
        ImGui::DestroyContext();
    }
}

void Framework::run_imgui_frame(bool from_present)
{
    std::scoped_lock _{m_imgui_mtx};

    m_has_frame = false;

    if (!m_initialized)
    {
        return;
    }

    const bool is_init_ok = m_error.empty() && m_game_data_initialized;

    consume_input();
    update_fonts();

    ImGui_ImplWin32_NewFrame();

    // from_present is so we don't accidentally
    // run script/game code within the present thread.
    if (is_init_ok && !from_present)
    {
        // Run mod frame callbacks.
        // m_mods->on_pre_imgui_frame();
    }

    ImGui::NewFrame();

    if (!from_present)
    {
        call_on_frame();
    }

    draw_ui();
    m_last_draw_ui = m_draw_ui;

    ImGui::EndFrame();
    ImGui::Render();

    m_has_frame = true;
}

// D3D12 Draw funciton
void Framework::on_frame_d3d12()
{
    std::scoped_lock _{m_imgui_mtx};

    auto command_queue = m_d3d12_hook->get_command_queue();
    // spdlog::debug("on_frame (D3D12)");

    if (!m_initialized)
    {
        if (!initialize())
        {
            spdlog::error("Failed to initialize Framework on DirectX 12");
            return;
        }

        spdlog::info("Framework initialized");
        m_initialized = true;
        return;
    }

    if (command_queue == nullptr)
    {
        spdlog::error("Null Command Queue");
        return;
    }

    if (m_message_hook_requested)
    {
        initialize_windows_message_hook();
        initialize_xinput_hook();
        initialize_dinput_hook();
    }

    auto device = m_d3d12_hook->get_device();

    if (device == nullptr)
    {
        spdlog::error("D3D12 Device was null when it shouldn't be, returning...");
        m_initialized = false;
        return;
    }

    bool is_init_ok = first_frame_initialize();

    // if (!m_has_frame) {
    // if (!is_init_ok) {
    // update_fonts();

    ImGui::GetIO().BackendRendererUserData = m_d3d12.imgui_backend_datas[0];
    const auto prev_cleanup = m_wants_device_object_cleanup;
    invalidate_device_objects();
    ImGui_ImplDX12_NewFrame();

    ImGui::GetIO().BackendRendererUserData = m_d3d12.imgui_backend_datas[1];
    m_wants_device_object_cleanup = prev_cleanup;
    invalidate_device_objects();
    ImGui_ImplDX12_NewFrame();
    // hooks don't run until after initialization, so we just render the imgui window while initalizing.
    run_imgui_frame(false);
    /*    } else {
            return;
        }
    } else {
        invalidate_device_objects();
        ImGui_ImplDX12_NewFrame();
    }*/

    // if (is_init_ok)
    // {
    //     m_mods->on_present();
    // }

    if (m_d3d12.cmd_ctxs.empty())
    {
        return;
    }

    auto &cmd_ctx = m_d3d12.cmd_ctxs[m_d3d12.cmd_ctx_index++ % m_d3d12.cmd_ctxs.size()];

    if (cmd_ctx == nullptr)
    {
        return;
    }

    cmd_ctx->wait(INFINITE);
    {
        std::scoped_lock _{cmd_ctx->mtx};
        cmd_ctx->has_commands = true;

        // Draw to our render target.
        D3D12_RESOURCE_BARRIER barrier{};
        barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
        barrier.Flags = D3D12_RESOURCE_BARRIER_FLAG_NONE;
        barrier.Transition.pResource = m_d3d12.get_rt(D3D12::RTV::IMGUI).Get();
        barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
        barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE;
        barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET;
        cmd_ctx->cmd_list->ResourceBarrier(1, &barrier);

        float clear_color[]{0.0f, 0.0f, 0.0f, 0.0f};
        D3D12_CPU_DESCRIPTOR_HANDLE rts[1]{};
        cmd_ctx->cmd_list->ClearRenderTargetView(m_d3d12.get_cpu_rtv(device, D3D12::RTV::IMGUI), clear_color, 0, nullptr);
        rts[0] = m_d3d12.get_cpu_rtv(device, D3D12::RTV::IMGUI);
        cmd_ctx->cmd_list->OMSetRenderTargets(1, rts, FALSE, NULL);
        cmd_ctx->cmd_list->SetDescriptorHeaps(1, m_d3d12.srv_desc_heap.GetAddressOf());

        ImGui::GetIO().BackendRendererUserData = m_d3d12.imgui_backend_datas[1];
        ImGui_ImplDX12_RenderDrawData(ImGui::GetDrawData(), cmd_ctx->cmd_list.Get());

        barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
        barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE;
        cmd_ctx->cmd_list->ResourceBarrier(1, &barrier);

        // Draw to the back buffer.
        auto swapchain = m_d3d12_hook->get_swap_chain();
        auto bb_index = swapchain->GetCurrentBackBufferIndex();
        barrier.Transition.pResource = m_d3d12.rts[bb_index].Get();
        barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_PRESENT;
        barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET;
        cmd_ctx->cmd_list->ResourceBarrier(1, &barrier);
        rts[0] = m_d3d12.get_cpu_rtv(device, (D3D12::RTV)bb_index);
        cmd_ctx->cmd_list->OMSetRenderTargets(1, rts, FALSE, NULL);
        cmd_ctx->cmd_list->SetDescriptorHeaps(1, m_d3d12.srv_desc_heap.GetAddressOf());

        ImGui::GetIO().BackendRendererUserData = m_d3d12.imgui_backend_datas[0];
        ImGui_ImplDX12_RenderDrawData(ImGui::GetDrawData(), cmd_ctx->cmd_list.Get());

        barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
        barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_PRESENT;
        cmd_ctx->cmd_list->ResourceBarrier(1, &barrier);

        cmd_ctx->execute();
    }
}

void Framework::on_post_present_d3d12()
{
    if (!m_error.empty() || !m_initialized || !m_game_data_initialized)
    {
        if (m_last_present_time <= std::chrono::steady_clock::now())
        {
            m_last_present_time = std::chrono::steady_clock::now();
        }

        return;
    }

    if (m_last_present_time <= std::chrono::steady_clock::now())
    {
        m_last_present_time = std::chrono::steady_clock::now();
    }
}

void Framework::on_reset(uint32_t w, uint32_t h)
{
    std::scoped_lock _{m_imgui_mtx};

    spdlog::info("Reset! {} {}", w, h);

    if (m_initialized)
    {
        // fixes text boxes not being able to receive input
        imgui::reset_keystates();
    }

    deinit_d3d12();
    m_d3d12.rt_width = w;
    m_d3d12.rt_height = h;

    m_has_frame = false;
    m_first_initialize = false;
    m_initialized = false;
}

void Framework::post_message(UINT message, WPARAM w_param, LPARAM l_param)
{
    if (m_wnd == nullptr)
    {
        return;
    }

    PostMessage(m_wnd, message, w_param, l_param);
}

bool Framework::on_message(HWND wnd, UINT message, WPARAM w_param, LPARAM l_param)
{
    m_last_message_time = std::chrono::steady_clock::now();

    if (!m_initialized)
    {
        return true;
    }

    // If we called is_filtered during a WM_GETTEXT message we would deadlock.
    if (message != WM_GETTEXT && !WindowFilter::get().is_filtered(wnd))
    {
        // m_uevr_shared_memory->data().main_thread_id = GetCurrentThreadId();
    }

    bool is_mouse_moving{false};
    switch (message)
    {
    case WM_KEYDOWN:
    case WM_SYSKEYDOWN:
        if (w_param >= 0 && w_param < 256)
        {
            m_last_keys[w_param] = true;
        }

        if (w_param == VK_INSERT)
        {
            set_draw_ui(!m_draw_ui, true);
            return false;
        }
        break;
    case WM_KEYUP:
    case WM_SYSKEYUP:
        if (w_param >= 0 && w_param < 256)
        {
            m_last_keys[w_param] = false;
        }
        break;
    case WM_KILLFOCUS:
        std::fill(std::begin(m_last_keys), std::end(m_last_keys), false);
        break;
    case WM_SETFOCUS:
        std::fill(std::begin(m_last_keys), std::end(m_last_keys), false);
        break;
    case WM_ACTIVATE:
        if (LOWORD(w_param) == WA_INACTIVE)
        {
            return false;
        }
        break;
    case WM_INPUT:
    {
        // RIM_INPUT means the window has focus
        /*if (GET_RAWINPUT_CODE_WPARAM(w_param) == RIM_INPUT) {
            uint32_t size = sizeof(RAWINPUT);
            RAWINPUT raw{};

            // obtain size
            GetRawInputData((HRAWINPUT)l_param, RID_INPUT, nullptr, &size, sizeof(RAWINPUTHEADER));

            auto result = GetRawInputData((HRAWINPUT)l_param, RID_INPUT, &raw, &size, sizeof(RAWINPUTHEADER));

            if (raw.header.dwType == RIM_TYPEMOUSE) {
                m_accumulated_mouse_delta[0] += (float)raw.data.mouse.lLastX;
                m_accumulated_mouse_delta[1] += (float)raw.data.mouse.lLastY;

                // Allowing camera movement when the UI is hovered while not focused
                if (raw.data.mouse.lLastX || raw.data.mouse.lLastY) {
                    is_mouse_moving = true;
                }
            }
        }*/
    }
    break;

    default:
        break;
    }

    ImGui_ImplWin32_WndProcHandler(wnd, message, w_param, l_param);

    {
        // If the user is interacting with the UI we block the message from going to the game.
        const auto &io = ImGui::GetIO();
        if (m_draw_ui && !m_ui_passthrough)
        {
            // Fix of a bug that makes the input key down register but the key up will never register
            // when clicking on the ui while the game is not focused
            if (message == WM_INPUT && GET_RAWINPUT_CODE_WPARAM(w_param) == RIM_INPUTSINK)
            {
                return false;
            }

            static std::unordered_set<UINT> forcefully_allowed_messages{
                WM_DEVICECHANGE, // Allows XInput devices to connect to UE
                WM_SHOWWINDOW,
                WM_ACTIVATE,
                WM_ACTIVATEAPP,
                WM_CLOSE,
                WM_DPICHANGED,
                WM_SIZING,
                WM_MOUSEACTIVATE};

            if (!forcefully_allowed_messages.contains(message))
            {
                if (m_is_ui_focused)
                {
                    if (io.WantCaptureMouse || io.WantCaptureKeyboard || io.WantTextInput)
                        return false;
                }
                else
                {
                    if (!is_mouse_moving && (io.WantCaptureMouse || io.WantCaptureKeyboard || io.WantTextInput))
                        return false;
                }
            }
        }
    }

    bool any_false = false;

    return !any_false;
}

// this is unfortunate.
void Framework::on_direct_input_keys(const std::array<uint8_t, 256> &keys)
{
    /*const auto menu_key = FrameworkConfig::get()->get_menu_key()->value();

    if (keys[menu_key] && m_last_keys[menu_key] == 0) {
        std::lock_guard _{m_input_mutex};

        set_draw_ui(!m_draw_ui);
    }

    m_last_keys = keys;*/
}

std::filesystem::path Framework::get_persistent_dir()
{
    auto return_appdata_dir = []() -> std::filesystem::path
    {
        wchar_t app_data_path[MAX_PATH]{};
        SHGetSpecialFolderPathW(0, app_data_path, CSIDL_APPDATA, false);

        const auto exe_name = [&]()
        {
            const auto result = std::filesystem::path(*utility::get_module_pathw(utility::get_executable())).stem().string();
            const auto dir = std::filesystem::path(app_data_path) / "UnrealVRMod" / result;
            std::filesystem::create_directories(dir);

            return result;
        }();

        return std::filesystem::path(app_data_path) / "UnrealVRMod" / exe_name;
    };

    static const auto result = return_appdata_dir();

    return result;
}

int Framework::add_font(const std::filesystem::path &filepath, int size, const std::vector<ImWchar> &ranges)
{
    // Look for a font already matching this description.
    for (int i = 0; i < m_additional_fonts.size(); ++i)
    {
        const auto &font = m_additional_fonts[i];

        if (font.filepath == filepath && font.size == size && font.ranges == ranges)
        {
            return i;
        }
    }

    m_additional_fonts.emplace_back(Framework::AdditionalFont{filepath, size, ranges, (ImFont *)nullptr});
    m_fonts_need_updating = true;

    return m_additional_fonts.size() - 1;
}

void Framework::update_fonts()
{
    if (!m_fonts_need_updating)
    {
        return;
    }

    m_fonts_need_updating = false;

    auto &fonts = ImGui::GetIO().Fonts;

    fonts->Clear();
    fonts->AddFontFromMemoryCompressedTTF(RobotoMedium_compressed_data, RobotoMedium_compressed_size, (float)m_font_size);

    for (auto &font : m_additional_fonts)
    {
        const ImWchar *ranges = nullptr;

        if (!font.ranges.empty())
        {
            ranges = font.ranges.data();
        }

        if (fs::exists(font.filepath))
        {
            font.font = fonts->AddFontFromFileTTF(font.filepath.string().c_str(), (float)font.size, nullptr, ranges);
        }
        else
        {
            font.font = fonts->AddFontFromMemoryCompressedTTF(RobotoMedium_compressed_data, RobotoMedium_compressed_size, (float)font.size, nullptr, ranges);
        }
    }

    fonts->Build();
    m_wants_device_object_cleanup = true;
}

void Framework::invalidate_device_objects()
{
    if (!m_wants_device_object_cleanup)
    {
        return;
    }

    ImGui_ImplDX12_InvalidateDeviceObjects();

    m_wants_device_object_cleanup = false;
}

void Framework::draw_ui()
{
    std::lock_guard _{m_input_mutex};

    ImGui::GetIO().ConfigFlags |= ImGuiConfigFlags_NoMouseCursorChange; // causes bugs with the cursor

    auto &io = ImGui::GetIO();

    if (io.WantCaptureKeyboard)
    {
        // m_dinput_hook->ignore_input();
    }
    else
    {
        // m_dinput_hook->acknowledge_input();
    }

    if (!m_last_draw_ui || m_cursor_state_changed)
    {
        m_cursor_state_changed = false;
    }

    static const auto UEVR_NAME = std::format("UEVR [{}+{}-{:.8}]");

    ImGui::Begin(UEVR_NAME.c_str(), &m_draw_ui);

    ImGui::BeginGroup();
    ImGui::Columns(2);
    ImGui::BeginGroup();

    ImGui::Checkbox("Transparency", &m_ui_option_transparent);
    ImGui::SameLine();
    ImGui::Text("(?)");
    if (ImGui::IsItemHovered())
    {
        ImGui::SetTooltip("Makes the UI transparent when not focused.");
    }
    ImGui::Checkbox("Input Passthrough", &m_ui_passthrough);
    ImGui::SameLine();
    ImGui::Text("(?)");
    if (ImGui::IsItemHovered())
    {
        ImGui::SetTooltip("Allows mouse and keyboard inputs to register to the game while the UI is focused.");
    }

    ImGui::SameLine();
    ImGui::Text("(?)");
    if (ImGui::IsItemHovered())
    {
        ImGui::SetTooltip("Show additional options for greater control over various settings.");
    }

    ImGui::EndGroup();
    ImGui::NextColumn();

    ImGui::BeginGroup();
    ImGui::Text("Keyboard Menu Key: Insert");
    ImGui::Text("Gamepad L3 + R3: Toggle Menu");
    ImGui::Text("Gamepad RT: Shortcuts");
    ImGui::Text("Gamepad LB/RB: Change Sidebar Page");

    ImGui::EndGroup();
    ImGui::EndGroup();

    ImGui::Columns(1);
    ImGui::End();
}

bool Framework::initialize()
{
    if (m_initialized)
    {
        return true;
    }

    if (m_first_initialize)
    {
        m_frames_since_init = 0;
        m_first_initialize = false;
    }

    if (m_frames_since_init < 60)
    {
        m_frames_since_init++;
        return false;
    }

    spdlog::info("Attempting to initialize DirectX 12");

    if (!m_d3d12_hook->is_hooked())
    {
        return false;
    }

    auto device = m_d3d12_hook->get_device();
    auto swap_chain = m_d3d12_hook->get_swap_chain();

    if (device == nullptr || swap_chain == nullptr)
    {
        m_first_initialize = true;

        spdlog::info("Device: {:x}", (uintptr_t)device);
        spdlog::info("SwapChain: {:x}", (uintptr_t)swap_chain);

        spdlog::info("Device or SwapChain null. DirectX 11 may be in use. Unhooking D3D12...");

        // We unhook D3D12
        if (m_d3d12_hook->unhook())
            spdlog::info("D3D12 unhooked!");
        else
            spdlog::error("Cannot unhook D3D12, this might crash.");

        m_valid = false;

        return false;
    }

    DXGI_SWAP_CHAIN_DESC swap_desc{};
    swap_chain->GetDesc(&swap_desc);

    m_wnd = swap_desc.OutputWindow;

    ImGui::CreateContext();

    static const auto imgui_ini = (get_persistent_dir() / "imgui.ini").string();
    ImGui::GetIO().IniFilename = imgui_ini.c_str();

    if (!ImGui_ImplWin32_Init(m_wnd))
    {
        spdlog::error("Failed to initialize ImGui ImplWin32.");
        return false;
    }

    if (!init_d3d12())
    {
        spdlog::error("Failed to init D3D12.");
        return false;
    }

    initialize_windows_message_hook();
    initialize_xinput_hook();
    initialize_dinput_hook();

    // TODO
    /*if (m_first_frame) {
        m_dinput_hook = std::make_unique<DInputHook>(m_wnd);
    } else {
        m_dinput_hook->set_window(m_wnd);
    }*/

    if (m_first_frame)
    {
        m_first_frame = false;

        spdlog::info("Starting game data initialization thread");
    }

    return true;
}

bool Framework::initialize_windows_message_hook()
{
    if (m_wnd == 0)
    {
        return false;
    }

    if (m_first_frame || m_message_hook_requested || m_windows_message_hook == nullptr)
    {
        m_last_message_time = std::chrono::steady_clock::now();
        m_windows_message_hook.reset();
        m_windows_message_hook = std::make_unique<WindowsMessageHook>(m_wnd);
        m_windows_message_hook->on_message = [this](auto wnd, auto msg, auto w_param, auto l_param)
        {
            return on_message(wnd, msg, w_param, l_param);
        };

        m_message_hook_requested = false;
        return true;
    }

    m_message_hook_requested = false;
    return false;
}

bool Framework::initialize_xinput_hook()
{
    if (m_first_frame || m_xinput_hook == nullptr)
    {
        m_xinput_hook.reset();
        m_xinput_hook = std::make_unique<XInputHook>();
    }

    return true;
}

bool Framework::initialize_dinput_hook()
{
    if (m_first_frame || m_dinput_hook == nullptr)
    {
        m_dinput_hook.reset();
        m_dinput_hook = std::make_unique<DInputHook>();
    }

    return true;
}

// Ran on the first valid frame after pre-initialization of mods has taken place and hasn't failed
// This one allows mods to run any initialization code in the context of the D3D thread (like VR code)
// It also is the one that actually loads any config files
bool Framework::first_frame_initialize()
{
    const bool is_init_ok = m_error.empty() && m_game_data_initialized;

    if (!is_init_ok || !m_first_frame_d3d_initialize)
    {
        return is_init_ok;
    }

    std::scoped_lock _{get_hook_monitor_mutex()};

    spdlog::info("Running first frame D3D initialization of mods...");

    m_first_frame_d3d_initialize = false;

    return true;
}

void Framework::call_on_frame()
{
    const bool is_init_ok = m_error.empty() && m_game_data_initialized;

    // if (is_init_ok)
    // {
    //     // Run mod frame callbacks.
    //     m_frame_worker->execute();
    //     m_mods->on_frame();
    // }
}
// DirectX 12 Initialization methods

bool Framework::init_d3d12()
{
    deinit_d3d12();

    auto device = m_d3d12_hook->get_device();

    spdlog::info("[D3D12] Creating command allocator...");

    m_d3d12.cmd_ctxs.clear();

    for (auto i = 0; i < 3; ++i)
    {
        auto &ctx = m_d3d12.cmd_ctxs.emplace_back(std::make_unique<d3d12::CommandContext>());

        if (!ctx->setup(L"Framework::m_d3d12.cmd_ctx"))
        {
            spdlog::error("[D3D12] Failed to create command context.");
            return false;
        }
    }

    spdlog::info("[D3D12] Creating RTV descriptor heap...");

    {
        D3D12_DESCRIPTOR_HEAP_DESC desc{};

        desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
        desc.NumDescriptors = (int)D3D12::RTV::COUNT;
        desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
        desc.NodeMask = 1;

        if (FAILED(device->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&m_d3d12.rtv_desc_heap))))
        {
            spdlog::error("[D3D12] Failed to create RTV descriptor heap.");
            return false;
        }

        m_d3d12.rtv_desc_heap->SetName(L"Framework::m_d3d12.rtv_desc_heap");
    }

    spdlog::info("[D3D12] Creating SRV descriptor heap...");

    {
        D3D12_DESCRIPTOR_HEAP_DESC desc{};

        desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;
        desc.NumDescriptors = (int)D3D12::SRV::COUNT;
        desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE;

        if (FAILED(device->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&m_d3d12.srv_desc_heap))))
        {
            spdlog::error("[D3D12] Failed to create SRV descriptor heap.");
            return false;
        }

        m_d3d12.srv_desc_heap->SetName(L"Framework::m_d3d12.srv_desc_heap");
    }

    spdlog::info("[D3D12] Creating render targets...");

    {
        // Create back buffer rtvs.
        auto swapchain = m_d3d12_hook->get_swap_chain();

        for (auto i = 0; i <= (int)D3D12::RTV::BACKBUFFER_3; ++i)
        {
            if (SUCCEEDED(swapchain->GetBuffer(i, IID_PPV_ARGS(&m_d3d12.rts[i]))))
            {
                device->CreateRenderTargetView(m_d3d12.rts[i].Get(), nullptr, m_d3d12.get_cpu_rtv(device, (D3D12::RTV)i));
            }
            else
            {
                spdlog::error("[D3D12] Failed to get back buffer for rtv.");
            }
        }

        // Create our imgui and blank rts.
        auto &backbuffer = m_d3d12.get_rt(D3D12::RTV::BACKBUFFER_0);
        auto desc = backbuffer->GetDesc();

        spdlog::info("[D3D12] Back buffer format is {}", desc.Format);

        D3D12_HEAP_PROPERTIES props{};
        props.Type = D3D12_HEAP_TYPE_DEFAULT;
        props.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
        props.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;

        auto d3d12_rt_desc = desc;
        d3d12_rt_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM; // For VR

        D3D12_CLEAR_VALUE clear_value{};
        clear_value.Format = d3d12_rt_desc.Format;

        if (FAILED(device->CreateCommittedResource(&props, D3D12_HEAP_FLAG_NONE, &d3d12_rt_desc, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, &clear_value,
                                                   IID_PPV_ARGS(&m_d3d12.get_rt(D3D12::RTV::IMGUI)))))
        {
            spdlog::error("[D3D12] Failed to create the imgui render target.");
            return false;
        }

        m_d3d12.get_rt(D3D12::RTV::IMGUI)->SetName(L"Framework::m_d3d12.rts[IMGUI]");

        if (FAILED(device->CreateCommittedResource(&props, D3D12_HEAP_FLAG_NONE, &d3d12_rt_desc, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, &clear_value,
                                                   IID_PPV_ARGS(&m_d3d12.get_rt(D3D12::RTV::BLANK)))))
        {
            spdlog::error("[D3D12] Failed to create the blank render target.");
            return false;
        }

        m_d3d12.get_rt(D3D12::RTV::BLANK)->SetName(L"Framework::m_d3d12.rts[BLANK]");

        // Create imgui and blank rtvs and srvs.
        device->CreateRenderTargetView(m_d3d12.get_rt(D3D12::RTV::IMGUI).Get(), nullptr, m_d3d12.get_cpu_rtv(device, D3D12::RTV::IMGUI));
        device->CreateRenderTargetView(m_d3d12.get_rt(D3D12::RTV::BLANK).Get(), nullptr, m_d3d12.get_cpu_rtv(device, D3D12::RTV::BLANK));
        device->CreateShaderResourceView(
            m_d3d12.get_rt(D3D12::RTV::IMGUI).Get(), nullptr, m_d3d12.get_cpu_srv(device, D3D12::SRV::IMGUI_VR));
        device->CreateShaderResourceView(m_d3d12.get_rt(D3D12::RTV::BLANK).Get(), nullptr, m_d3d12.get_cpu_srv(device, D3D12::SRV::BLANK));

        m_d3d12.rt_width = (uint32_t)desc.Width;
        m_d3d12.rt_height = (uint32_t)desc.Height;

        m_last_rt_size = {desc.Width, desc.Height};
    }

    spdlog::info("[D3D12] Initializing ImGui...");

    auto &bb = m_d3d12.get_rt(D3D12::RTV::BACKBUFFER_0);
    auto bb_desc = bb->GetDesc();

    if (!ImGui_ImplDX12_Init(device, 3, bb_desc.Format, m_d3d12.srv_desc_heap.Get(),
                             m_d3d12.get_cpu_srv(device, D3D12::SRV::IMGUI_FONT_BACKBUFFER), m_d3d12.get_gpu_srv(device, D3D12::SRV::IMGUI_FONT_BACKBUFFER)))
    {
        spdlog::error("[D3D12] Failed to initialize ImGui.");
        return false;
    }

    m_d3d12.imgui_backend_datas[0] = ImGui::GetIO().BackendRendererUserData;

    ImGui::GetIO().BackendRendererUserData = nullptr;

    // Now initialize another one for the VR texture.
    auto &bb_vr = m_d3d12.get_rt(D3D12::RTV::IMGUI);
    auto bb_vr_desc = bb_vr->GetDesc();

    if (!ImGui_ImplDX12_Init(device, 3, bb_vr_desc.Format, m_d3d12.srv_desc_heap.Get(),
                             m_d3d12.get_cpu_srv(device, D3D12::SRV::IMGUI_FONT_VR), m_d3d12.get_gpu_srv(device, D3D12::SRV::IMGUI_FONT_VR)))
    {
        spdlog::error("[D3D12] Failed to initialize ImGui.");
        return false;
    }

    m_d3d12.imgui_backend_datas[1] = ImGui::GetIO().BackendRendererUserData;

    return true;
}

void Framework::deinit_d3d12()
{
    for (auto &ctx : m_d3d12.cmd_ctxs)
    {
        if (ctx != nullptr)
        {
            ctx->reset();
        }
    }

    m_d3d12.cmd_ctxs.clear();

    for (auto userdata : m_d3d12.imgui_backend_datas)
    {
        if (userdata != nullptr)
        {
            ImGui::GetIO().BackendRendererUserData = userdata;
            ImGui_ImplDX12_Shutdown();
        }
    }

    ImGui::GetIO().BackendRendererUserData = nullptr;
    m_d3d12 = {};
}
