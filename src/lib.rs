use game::Game;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};

mod game;
mod memedit;
mod render;
mod tools;
mod widgets;

#[no_mangle]
pub unsafe extern "stdcall" fn update(
    level: i32,
    paused: bool,
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
) {
    let angle = angle + 180.0 + 90.0;
    let angle = if angle > 360.0 { angle - 360.0 } else { angle };
    Game::update(level, paused, x, y, z, angle);
}

#[no_mangle]
pub unsafe extern "stdcall" fn toggle() {
    Game::toggle();
}

#[no_mangle]
pub unsafe extern "stdcall" fn toggle_big_map() {
    Game::toggle_big_map();
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    hmodule: HINSTANCE,
    reason: u32,
    _: *mut ::std::ffi::c_void,
) {
    if reason == DLL_PROCESS_ATTACH {
        Game::init();
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
