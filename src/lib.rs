use std::fs::File;
use std::{ffi, panic, thread, time};

use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::tracing;
use hudhook::windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};
use utils::{get_dll_dir, setup_tracing};

mod render;
mod utils;
mod wukong;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        // 初始化日志系统，输出到文件和控制台
        setup_tracing();

        tracing::info!("DllMain: DLL_PROCESS_ATTACH");

        // 设置panic钩子
        panic::set_hook(Box::new(|panic_info| {
            // 将panic信息记录到日志中
            if let Some(location) = panic_info.location() {
                tracing::error!(
                    "程序panic在 {}:{}:\n{}",
                    location.file(),
                    location.line(),
                    panic_info
                );
            } else {
                tracing::error!("程序panic: {}", panic_info);
            }
        }));

        thread::spawn(move || {
            // 延迟 10 秒启动
            thread::sleep(time::Duration::from_secs(10));
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
