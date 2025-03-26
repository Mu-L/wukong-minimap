use std::fs::File;
use std::{ffi, panic, thread, time};

use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};
use simplelog::*;
use utils::get_dll_dir;

mod render;
mod utils;
mod wukong;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        // 创建自定义配置
        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Debug,
                ConfigBuilder::new()
                    .set_time_offset_to_local()
                    .unwrap()
                    .build(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Debug,
                ConfigBuilder::new()
                    .set_time_offset_to_local()
                    .unwrap()
                    .build(),
                File::create(get_dll_dir().join("wukong_minimap.log")).unwrap(),
            ),
        ])
        .unwrap();

        // 设置panic钩子
        panic::set_hook(Box::new(|panic_info| {
            // 将panic信息记录到日志中
            if let Some(location) = panic_info.location() {
                log::error!(
                    "程序panic在 {}:{}:\n{}",
                    location.file(),
                    location.line(),
                    panic_info
                );
            } else {
                log::error!("程序panic: {}", panic_info);
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
                log::error!("Couldn't apply hooks: {e:?}");
                ::hudhook::eject();
            }
        });
    }
}
