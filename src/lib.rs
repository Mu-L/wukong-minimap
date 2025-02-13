use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::tracing;
use hudhook::windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};

mod render;
mod utils;
mod wukong;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut ::std::ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        // utils::setup_tracing();

        tracing::trace!("DllMain()");
        ::std::thread::spawn(move || {
            // 延迟 10 秒启动
            ::std::thread::sleep(::std::time::Duration::from_secs(10));
            if let Err(e) = ::hudhook::Hudhook::builder()
                .with::<ImguiDx12Hooks>(render::MiniMap::new())
                .with_hmodule(hmodule)
                .build()
                .apply()
            {
                tracing::error!("Couldn't apply hooks: {e:?}");
                ::hudhook::eject();
            }
        });
    }
}
