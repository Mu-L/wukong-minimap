use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};

mod game;
mod memedit;
mod render;
mod tools;
mod widgets;

#[no_mangle]
pub unsafe extern "stdcall" fn SetMapStatus(
    map_key: i32,
    status: bool,
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
) {
    // 实现设置玩家生命值的逻辑
    println!(
        "SetMapStatus {:?} {:?} {:?} {:?} {:?}  {:?}",
        map_key, status, x, y, z, angle
    );
    render::game
        .lock()
        .unwrap()
        .update(map_key, status, x, y, z, angle);
}

#[no_mangle]
pub unsafe extern "stdcall" fn toggle() {
    // 实现设置玩家生命值的逻辑
    println!("toggle");
    render::game.lock().unwrap().toggle();
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    hmodule: HINSTANCE,
    reason: u32,
    _: *mut ::std::ffi::c_void,
) {
    if reason == DLL_PROCESS_ATTACH {
        // tools::setup_tracing();
        ::hudhook::tracing::trace!("DllMain()");
        ::std::thread::spawn(move || {
            if let Err(e) = ::hudhook::Hudhook::builder()
                .with::<ImguiDx12Hooks>(render::MapHud::new())
                .with_hmodule(hmodule)
                .build()
                .apply()
            {
                ::hudhook::tracing::error!("Couldn't apply hooks: {e:?}");
                ::hudhook::eject();
            }
        });
    }
}
