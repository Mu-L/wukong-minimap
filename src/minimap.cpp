#include "minimap.h"
#include "ImGui/imgui.h"
#include "draw.h"
#include "texture_manager.h"
#include "helper.hpp"
#include "SDK/SDK/b1MinusManaged_classes.hpp"
#include "json.hpp"

#include "MinHook.h"

using pevent_fn = void(__thiscall *)(SDK::UObject *, SDK::UObject *, void *params);
inline static pevent_fn processEventOriginal = 0;

namespace Minimap
{
  // size
  static float size = 0.2f;
  // zoom
  static float zoom = 0.1f;

  // std::vector<MapInfo>
  static std::vector<MapInfo> maps = {};
  // map
  static MapInfo *current_map = nullptr;
  // info
  static PlayerInfo player_info = {
      -1.0f, // x
      -1.0f, // y
      -1.0f, // z
      0.0f,  // angle
      0,     // bIsLocalViewTarget
      ""};

  static void process_event_hook(SDK::UObject *caller, SDK::UObject *fn, void *params)
  {
  }
  /**
   * 读取 json 文件中的地图数据 当前目录下的 maps/data.json
   */
  std::vector<MapInfo> load_data()
  {
    std::string filename = "maps/data.json";
    std::string jsonStr = "";
    std::ifstream file(filename);
    if (file.is_open())
    {
      std::string line;
      while (std::getline(file, line))
      {
        jsonStr += line;
      }
      file.close();
    }
    try
    {
      auto j = nlohmann::json::parse(jsonStr);
      return j.get<std::vector<MapInfo>>();
    }
    catch (const std::exception &e)
    {
      // Handle parsing error
      return {};
    }
  }
  void update_map()
  {
    PlayerInfo &info = player_info;
    MapInfo *map = nullptr;
    bool found = false;
    for (auto it = maps.rbegin(); it != maps.rend() && !found; ++it)
    {
      MapInfo &m = *it;
      if (info.level == m.level &&
          info.x >= m.range.start[0] && info.x <= m.range.end[0] &&
          info.y >= m.range.start[1] && info.y <= m.range.end[1] &&
          info.z >= m.range.start[2] && info.z <= m.range.end[2])
      {
        if (m.areas.empty())
        {
          map = &m;
          found = true;
          break;
        }
        for (auto &area : m.areas)
        {
          if (info.x >= area.start[0] && info.x <= area.end[0] &&
              info.y >= area.start[1] && info.y <= area.end[1] &&
              info.z >= area.start[2] && info.z <= area.end[2])
          {
            map = &m;
            found = true;
            break;
          }
        }
      }
    }
    if (map)
    {
      if (!current_map || current_map->key != map->key)
      {
        current_map = map;
        printf("load map: %s\n", map->key.c_str());
        UnloadTextureFormFile("map", ("./maps/" + map->key).c_str());
      }
    }
    else
    {
      current_map = nullptr;
    }
  }

  void update_info()
  {
    PlayerInfo info = {
        -1.0f, // x
        -1.0f, // y
        -1.0f, // z
        0.0f,  // angle
        0,     // bIsLocalViewTarget
        ""};

    SDK::UWorld *World = SDK::UWorld::GetWorld();
    if (!World)
    {
      player_info.bIsLocalViewTarget = 0;
      return;
    }
    SDK::UGameplayStatics *GameplayStatics = SDK::UGameplayStatics::GetDefaultObj();
    if (!GameplayStatics)
    {
      player_info.bIsLocalViewTarget = 0;
      return;
    }

    SDK::ACharacter *playerCharacter = SDK::UBGUFunctionLibrary::GetPlayerCharacter(World);
    if (!playerCharacter)
    {
      player_info.bIsLocalViewTarget = 0;
      return;
    }

    // 获取当前关卡名称
    std::string currentLevelName(GameplayStatics->GetCurrentLevelName(World, false).ToString());
    info.level = currentLevelName;
    // 获取位置和角度
    SDK::FVector Location = playerCharacter->K2_GetActorLocation();
    SDK::FRotator Rotator = playerCharacter->K2_GetActorRotation();

    info.x = Location.X;
    info.y = Location.Y;
    info.z = Location.Z;

    float angle = Rotator.Yaw + 90.0;
    info.angle = angle < 0.0 ? angle + 360.0 : angle;

    info.bIsLocalViewTarget = playerCharacter->bIsLocalViewTarget;
    player_info = info;
  }
  ImVec2 location_to_map(ImVec2 pos)
  {
    if (!current_map)
    {
      return ImVec2(0.5f, 0.5f);
    }
    MapInfo &map = *current_map;
    float x_start = current_map->range.start[0];
    float y_start = current_map->range.start[1];
    float x_end = current_map->range.end[0];
    float y_end = current_map->range.end[1];
    float x_offset = (pos.x - x_start) / (x_end - x_start);
    float y_offset = (pos.y - y_start) / (y_end - y_start);
    return ImVec2(x_offset, y_offset);
  }

  std::pair<ImVec2, ImVec2> get_map_uv(ImVec2 pos)
  {
    ImVec2 map_pos = location_to_map(pos);
    float half_zoom = zoom / 2.0f;
    ImVec2 min_pos = ImVec2(map_pos.x - half_zoom, map_pos.y - half_zoom);
    ImVec2 max_pos = ImVec2(map_pos.x + half_zoom, map_pos.y + half_zoom);
    return std::make_pair(min_pos, max_pos);
  }
  std::pair<ImVec2, ImVec2> get_icon_pos(ImVec2 pos, ImVec2 player_pos, ImVec2 center, float view_size)
  {
    float x_start = current_map->range.start[0];
    float y_start = current_map->range.start[1];
    float x_end = current_map->range.end[0];
    float y_end = current_map->range.end[1];
    float x_scale = view_size / (abs(x_end - x_start) * zoom);
    float y_scale = view_size / (abs(y_end - y_start) * zoom);
    ImVec2 new_pos = ImVec2(
        center.x + (pos.x - player_pos.x) * x_scale,
        center.y + (pos.y - player_pos.y) * y_scale);
    ImVec2 min_pos = ImVec2(new_pos.x - 16.0f, new_pos.y - 16.0f);
    ImVec2 max_pos = ImVec2(new_pos.x + 16.0f, new_pos.y + 16.0f);
    return std::make_pair(min_pos, max_pos);
  }
  std::array<ImVec2, 4> arrow_to_p4(ImVec2 location, float size, float angle)
  {
    float rad = angle * (3.14159265358979323846f / 180.0f); // Convert angle to radians
    float sin = std::sin(rad);
    float cos = std::cos(rad);
    float half = size / 2.0f;

    auto transform = [&](float dx, float dy) -> ImVec2
    {
      return ImVec2(
          location.x + dx * cos - dy * sin,
          location.y + dx * sin + dy * cos);
    };

    return {
        transform(-half, -half), // Top-left
        transform(half, -half),  // Top-right
        transform(half, half),   // Bottom-right
        transform(-half, half)   // Bottom-left
    };
  }
  void init()
  {
    HMODULE baseModule = GetModuleHandle(NULL);
    while (!baseModule)
    {
      baseModule = GetModuleHandle(NULL);
      Sleep(1000);
    }
    // GObjects
    uint8_t *GObjectsScanResult = Memory::PatternScan(baseModule, "48 8B ?? ?? ?? ?? ?? 48 8B ?? ?? 48 8D ?? ?? EB ?? 33 ?? 8B ?? ?? C1 ??");
    if (GObjectsScanResult)
    {
      // spdlog::info("Offsets: GObjects: Address is {:s}+{:x}", sExeName.c_str(), (uintptr_t)GObjectsScanResult - (uintptr_t)baseModule);
      uintptr_t GObjectsAddr = Memory::GetAbsolute((uintptr_t)GObjectsScanResult + 0x3);
      SDK::Offsets::GObjects = (uintptr_t)GObjectsAddr - (uintptr_t)baseModule;
      // spdlog::info("Offsets: GObjects: Offset: {:x}", SDK::Offsets::GObjects);
    }
    else if (!GObjectsScanResult)
    {
      // spdlog::error("Offsets: GObjects: Pattern scan failed.");
    }

    // AppendString
    uint8_t *AppendStringScanResult = Memory::PatternScan(baseModule, "48 89 ?? ?? ?? 57 48 83 ?? ?? 80 3D ?? ?? ?? ?? 00 48 8B ?? 48 8B ?? 74 ?? 4C 8D ?? ?? ?? ?? ?? EB ?? 48 8D ?? ?? ?? ?? ?? E8 ?? ?? ?? ?? 4C ?? ?? C6 ?? ?? ?? ?? ?? 01 8B ?? 8B ?? 0F ?? ?? C1 ?? 10 89 ?? ?? ?? 89 ?? ?? ?? 48 8B ?? ?? ?? 48 ?? ?? ?? 8D ?? ?? 49 ?? ?? ?? ?? 48 8B ?? E8 ?? ?? ?? ?? 83 ?? ?? 00");
    if (AppendStringScanResult)
    {
      // spdlog::info("Offsets: AppendString: Address is {:s}+{:x}", sExeName.c_str(), (uintptr_t)AppendStringScanResult - (uintptr_t)baseModule);
      SDK::Offsets::AppendString = (uintptr_t)AppendStringScanResult - (uintptr_t)baseModule;
      // spdlog::info("Offsets: AppendString: Offset: {:x}", SDK::Offsets::AppendString);
    }
    else if (!AppendStringScanResult)
    {
      // spdlog::error("Offsets: AppendString: Pattern scan failed.");
    }

    uint8_t *ProcessEventScanResult = Memory::PatternScan(baseModule, "40 ?? 56 57 41 ?? 41 ?? 41 ?? 41 ?? 48 ?? ?? ?? ?? ?? ?? 48 8D ?? ?? ?? 48 89 ?? ?? ?? ?? ?? 48 8B ?? ?? ?? ?? ?? 48 ?? ?? 48 89 ?? ?? ?? ?? ?? 4D ?? ?? 48 ?? ?? 4C ?? ?? 48 ?? ??");
    if (ProcessEventScanResult)
    {
      SDK::Offsets::ProcessEvent = (UC::int32)((uintptr_t)ProcessEventScanResult - (uintptr_t)baseModule);

      // MH_STATUS ret = MH_CreateHook(reinterpret_cast<LPVOID>(ProcessEventScanResult), &process_event_hook, reinterpret_cast<void**>(&processEventOriginal));
    }

    // 从资源加载 JSON 数据
    maps = load_data();

    LoadTextureFromFile("map", "./maps/img_map_101.png");
    LoadTextureFromFile("player", "./maps/player.png");
    LoadTextureFromFile("teleport", "./maps/teleport.png");
  }
  void draw()
  {
    ImGuiIO &io = ImGui::GetIO();
    ImVec2 screen = io.DisplaySize;
    float map_size = min(screen.x, screen.y);
    // let [screen_width, screen_height] = ui.io().display_size;
    // let window_size = (screen_width * self.size).min(screen_height * self.size);
    // let [offset_x, offset_y] = [screen_width - window_size - 10.0, 10.0];
    float window_size = min(screen.x * size, screen.y * size);
    ImVec2 offset = ImVec2(screen.x - window_size - 10.0f, 10.0f);
    bool open = true;

    update_info();
    update_map();
    if (current_map && player_info.bIsLocalViewTarget)
    {
      ImGui::PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(0, 0));
      ImGui::PushStyleVar(ImGuiStyleVar_WindowRounding, 10.0f);
      ImGui::SetNextWindowSize(ImVec2(window_size, window_size));
      ImGui::SetNextWindowPos(offset);

      // 禁止移动，不要边框和标题栏，透明背景
      ImGui::Begin("minimap", &open,
                   ImGuiWindowFlags_NoDecoration |
                       ImGuiWindowFlags_NoNav |
                       ImGuiWindowFlags_NoInputs |
                       ImGuiWindowFlags_NoBackground);

      ImGui::PopStyleVar(2);
      auto [uv0, uv1] = get_map_uv(ImVec2(player_info.x, player_info.y));

      ImTextureID texId = TextureManager::Instance().GetTextureID("map");
      ImGui::Image(texId, ImVec2(window_size, window_size), uv0, uv1);

      ImVec2 center = ImVec2(offset.x + window_size / 2.0f, offset.y + window_size / 2.0f);
      ImTextureID teleportTexId = TextureManager::Instance().GetTextureID("teleport");
      for (const auto &point : current_map->points)
      {
        if (point.icon == 0)
        {
          auto [min, max] = get_icon_pos(ImVec2(point.x, point.y), ImVec2(player_info.x, player_info.y), center, window_size);
          ImGui::GetWindowDrawList()->AddImage(teleportTexId, min, max);
        }
      }
      auto points = arrow_to_p4(center, 32.0f, player_info.angle);
      ImTextureID playerTexId = TextureManager::Instance().GetTextureID("player");
      ImGui::GetWindowDrawList()->AddImageQuad(playerTexId, points[0], points[1], points[2], points[3]);

      ImGui::End();
      ImGui::SetNextWindowSize(ImVec2(window_size, 40));
      ImGui::SetNextWindowPos(ImVec2(offset.x, offset.y + window_size + 4.0f));
      ImGui::Begin("info", &open,
                   ImGuiWindowFlags_NoDecoration |
                       ImGuiWindowFlags_NoNav |
                       ImGuiWindowFlags_NoInputs |
                       ImGuiWindowFlags_NoBackground);
      const char *text = "bilibili: jaskang";
      float window_width = ImGui::GetWindowSize().x;
      float text_width = ImGui::CalcTextSize(text).x;
      ImGui::SetCursorPosX((window_width - text_width) * 0.5f);
      ImGui::Text("%s", text);
      ImGui::End();
    }
  }

}