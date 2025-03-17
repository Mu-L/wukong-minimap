use std::{ffi, thread, time};

use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::tracing;
use hudhook::windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};

mod render;
mod utils;
mod wukong;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        thread::spawn(move || {
            // utils::setup_tracing();
            tracing::trace!("DllMain()");
            // 延迟 10 秒启动
            thread::sleep(time::Duration::from_secs(5));
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
