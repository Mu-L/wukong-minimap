use hudhook::windows::{
    core::{s, PCSTR},
    Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA},
};
use libmem::{find_module, get_process, hook_code, read_memory, sig_scan_ex};
use std::f64::consts::PI;
use std::ffi::CString;

use crate::globals::{
    asm_map_hook, asm_pause_hook, asm_pos_hook, g_map_address, g_map_back_address, g_pause_address,
    g_pause_back_address, g_pos_address, g_pos_back_address,
};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct GameInfo {
    mapid: i32,
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
    playing: u8,
}

type GetGameInfoFn = unsafe extern "C" fn() -> GameInfo;

fn csharp_get_info() -> Option<GameInfo> {
    unsafe {
        // 获取已加载的 DLL 句柄
        let handle = GetModuleHandleA(s!("wukong.asi")).unwrap();
        // 获取函数地址
        let proc_addr = GetProcAddress(handle, s!("GetGameInfo"));
        if let Some(func) = proc_addr {
            // 转换为函数指针并调用
            let get_game_info: GetGameInfoFn = std::mem::transmute(func);
            Some(get_game_info())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    // 方向向量 2 个 f64
    pub angle_x: f64,
    pub angle_y: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub playing: bool,
    pub map_id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}

pub struct Wukong {}

impl Wukong {
    pub fn init() -> Self {
        let current_path = hudhook::util::get_dll_path().unwrap();
        let dll_path = current_path
            .parent()
            .unwrap()
            .join("assets")
            .join("wukong.asi");

        // 将 Path 转换为 CString
        let dll_path = dll_path.to_str().unwrap().to_owned();
        // 加载 wukong.asi
        unsafe { LoadLibraryA(PCSTR::from_raw(dll_path.as_ptr())).ok() };

        let is_csharp_loader = true;
        let game_process = get_process().unwrap();
        let game_module = find_module("b1-Win64-Shipping.exe").unwrap();
        // F2 0F 10 40 10 49 8B D7
        let map_hook_address = sig_scan_ex(
            &game_process,
            "48 8B 46 28 48 63 56 40 48 8B C8 4C 8B C5",
            game_module.base,
            game_module.size,
        )
        .unwrap();
        // 这个注释不能删除，否则会报错
        // hook_result 返回了 trampoline 结构体，里面有 address 和 size 两个字段，address 处保留了原始地址被jmp 覆盖的内容且附带了jmp 回去的指令
        let hook_result =
            unsafe { hook_code(map_hook_address, &asm_map_hook as *const _ as usize).unwrap() };
        // 将返回的地址保存到 g_map_back_address
        unsafe { g_map_back_address = hook_result.address };

        unsafe {
            println!(
                "map_hook_original: {:#X} map_hook_address: {:#X} g_map_back_address: {:#X}",
                map_hook_address, &asm_map_hook as *const _ as usize, g_map_back_address
            );
        }

        let pos_hook_address = sig_scan_ex(
            &game_process,
            "F2 0F 10 40 10 49 8B D7",
            game_module.base,
            game_module.size,
        )
        .unwrap();

        // 这个注释不能删除，否则会报错
        // hook_result 返回了 trampoline 结构体，里面有 address 和 size 两个字段，address 处保留了原始地址被jmp 覆盖的内容且附带了jmp 回去的指令
        let hook_result =
            unsafe { hook_code(pos_hook_address, &asm_pos_hook as *const _ as usize).unwrap() };
        // 将返回的地址保存到 g_map_back_address
        unsafe { g_pos_back_address = hook_result.address };

        unsafe {
            println!(
                "pos_hook_original: {:#X} pos_hook_address: {:#X} g_pos_back_address: {:#X}",
                pos_hook_address, &asm_pos_hook as *const _ as usize, g_pos_back_address
            );
        }

        let pause_hook_address = if is_csharp_loader {
            sig_scan_ex(
                &game_process,
                "49 8B 47 30 48 8B C8 49 8B D6 4C 8B C5 49 83 C0 C8 83 38 00",
                game_module.base,
                game_module.size,
            )
            .unwrap()
        } else {
            sig_scan_ex(
                &game_process,
                "89 47 48 48 63 47 4C FF C0 89 47 4C B8 01000000",
                game_module.base,
                game_module.size,
            )
            .unwrap()
        };

        // 这个注释不能删除，否则会报错
        // hook_result 返回了 trampoline 结构体，里面有 address 和 size 两个字段，address 处保留了原始地址被jmp 覆盖的内容且附带了jmp 回去的指令
        let hook_result = unsafe {
            hook_code(
                pause_hook_address + 0x4,
                &asm_pause_hook as *const _ as usize,
            )
            .unwrap()
        };
        // 将返回的地址保存到 g_map_back_address
        unsafe { g_pause_back_address = hook_result.address };

        unsafe {
            println!(
                "pause_hook_original: {:#X} pause_hook_address: {:#X} g_pause_back_address: {:#X}",
                pause_hook_address, &asm_pause_hook as *const _ as usize, g_pause_back_address
            );
        }

        Self {}
    }

    // 获取地图id
    pub fn game_state() -> GameState {
        let is_csharp_loader = true;
        if is_csharp_loader {
            let info = csharp_get_info().unwrap();
            return GameState {
                playing: info.playing == 1,
                map_id: info.mapid,
                angle: info.angle,
                x: info.x as f32,
                y: info.y as f32,
                z: info.z as f32,
            };
        }

        let map_id: i32 = match unsafe { g_map_address } {
            0 => 0,
            _ => unsafe { read_memory::<i32>(g_map_address + 0x40) },
        };

        let position = match unsafe { g_pos_address } {
            0 => Position {
                angle_x: 0.0,
                angle_y: 0.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            _ => unsafe { read_memory::<Position>(g_pos_address - 0x10) },
        };

        let palying = match unsafe { g_pause_address } {
            0 => 0,
            _ => unsafe { read_memory::<i32>(g_pause_address + 0x48) },
        };

        GameState {
            playing: palying != 0,
            map_id,
            angle: vector_to_angle(position.angle_x, position.angle_y),
            x: position.x as f32,
            y: position.y as f32,
            z: position.z as f32,
        }
    }
}

pub fn vector_to_angle(angle_x: f64, angle_y: f64) -> f32 {
    let angle_radians = angle_x.atan2(angle_y); // 使用 atan2 计算角度
    let angle_degrees = angle_radians * (360.0 / PI);
    let angle_degrees = angle_degrees - 90.0;
    if angle_degrees < 0.0 {
        (angle_degrees + 360.0) as f32
    } else if angle_degrees > 360.0 {
        (angle_degrees - 360.0) as f32
    } else {
        angle_degrees as f32
    }
}
