use hudhook::windows::{
    core::{s, PCSTR},
    Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA},
};
use libmem::{find_module, get_process, hook_code, read_memory, sig_scan_ex};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::OnceLock;

use crate::globals::{
    asm_map_hook, asm_pause_hook, asm_pos_hook, g_map_address, g_map_back_address, g_pause_address,
    g_pause_back_address, g_pos_address, g_pos_back_address,
};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct GameInfo {
    level_name: [u8; 256],
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
        let handle = GetModuleHandleA(s!("WukongApi.asi")).unwrap();
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
    pub level_name: String,
    pub map_id: i32,
    pub playing: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}

// [map]
// 场景ID
// 13=序章云端_MGD
// 10=黑风山
// 11=隐·旧观音禅院
// 12=黑风山-尺木间
// 20=黄风岭
// 25=隐·斯哈里国
// 24=黄风岭-藏龙洞
// 30=小西天
// 62=隐·梅山
// 92=小西天-浮屠塔
// 40=盘丝岭
// 80=隐·紫云山
// 50=火焰山
// 70=隐·壁水洞
// 98=花果山
// 61=石卵
// 31=如意画轴-六六村

// 103=除错房间
// 4444=测试空房间
// 4445=战斗记录地图
// 1=*开始界面*
// 3=***FTT_RZD_01
// 4=map_derek_test
// 5=map_hatum_test
// 6=map_shep_test
// 7=SetConfig_P
// 9=***花果山顶HGS01_PersistentLevel

// 21=***HFM_XiaoXuMi_01
// 22=***HFM_HouHu_Cave01
// 23=***Online_SHLG1_Persist

// 51=***HYS_RYGD_PersistentLevel
// 60=***EndA01_PersistentLevel
// 90=***FTT01_persistentlevel
// 91=***BDM
// 97=***两界山LJS_persistentlevel
// 99=***神之猎场SZLC01_persistentlevel
// 100=map_rebirth_test
// 101=CharlieTestOnline
// 102=map_cd9_fx_wlg
// 104=ONLINE_test
// 105=ONLINE_guansi_test01
// 106=Demo_Battle_Rongda
// 107=map_sybil_test
// 108=cd9测试
// 109=Demo_Battle_Final
// 110=Demo_Battle_Feiluo
// 111=***石敢当SGD01_persistentlevel
// 112=BYS_test
// 113=Demo_jasonwu
// 1000=Demo_Battle_scene00
// 3615=Demo_Battle_Ziheng
// 3616=map_chris_test_ex
// 3617=***Demo_Battle_Jeffrey
// 3618=***DebugRoom_Travel_Persistent
// 3619=DebugRoom_collection
// 3620=***DebugRoom_MoveMode
// 3621=map_jzptest_3
// 3622=***Test_SJJ
// 3623=***LHC_party_map
// 3624=Boss01_test_map
// 3625=Boss02_test_map
// 3626=Boss03_test_map
// 3627=map_migge_Collection
// 3630=***GamePlayTraceTest_Battlet
// 3631=guansi_tianbing
// 3632=fjay_test_scene_mini
// 3633=***test_XF
// 3634=Demo_Battle_Transform
// 3635=Demo_Battle_VigorSkill
// 3636=***guansi_tianjiang

// 使用 OnceLock 实现 LEVEL_ID_MAP
static LEVEL_ID_MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();

fn get_mapid_by_map(map_name: &str) -> i32 {
    let map = LEVEL_ID_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("LYS_paintingworld_01", 31); // 如意画轴-六六村
        m.insert("HFS01_PersistentLevel", 10); // 黑风山
        m.insert("HFS01_Old_GYCY_YKX_PersistentLevel", 11); // 隐·旧观音禅院
        m.insert("HFS_WoodDragon", 12); // 黑风山-尺木间
        m.insert("HFM02_PersistentLevel", 20); // 黄风岭
        m.insert("HFM_DuJiaoXian_Persist", 25); // 隐·斯哈里国
        m.insert("HFM_DustDragon_01", 24); // 黄风岭-藏龙洞
        m.insert("LYS_PersistentLevel", 30); // 小西天
        m.insert("PSD_PersistentLevel", 40); // 盘丝岭
        m.insert("ZYS01_persistentlevel", 80); // 隐·紫云山
        m.insert("HYS_PersistentLevel", 50); // 火焰山
        m.insert("BYS_persistentlevel", 98); // 花果山
        m.insert("BSD02_persistentlevel", 70); // 隐·壁水洞
        m
    });

    map.get(map_name).copied().unwrap_or(999)
}

pub struct Wukong {}

impl Wukong {
    pub fn init() -> Self {
        let current_path = hudhook::util::get_dll_path().unwrap();
        let dll_path = current_path
            .parent()
            .unwrap()
            .join("assets")
            .join("WukongApi.asi");

        // 将 Path 转换为 CString
        let dll_path = dll_path.to_str().unwrap().to_owned();
        // 加载 WukongApi.asi
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
            let level_name = String::from_utf8_lossy(&info.level_name)
                .trim_matches(char::from(0))
                .to_string();

            // 使用新的函数名获取地图ID
            let map_id = get_mapid_by_map(&level_name);
            let angle = info.angle - 90.0;
            let angle = if angle < 0.0 { angle + 360.0 } else { angle };
            return GameState {
                playing: info.playing == 1,
                level_name,
                map_id,
                angle,
                x: info.x,
                y: info.y,
                z: info.z,
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
            level_name: "".to_string(),
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
