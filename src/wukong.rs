use once_cell::sync::Lazy;
use std::sync::Mutex;

// 添加静态变量来存储上一次的状态
static LAST_STATE: Lazy<Mutex<Option<GameState>>> = Lazy::new(|| Mutex::new(None));

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct PlayerInfo {
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
    is_local_view_target: u8,
    level_name: [u8; 256],
}

// type GetGameInfoFn = unsafe extern "C" fn() -> GameInfo;

extern "C" {
    fn getPlayerInfo() -> PlayerInfo;
}

extern "C" {
    fn b1Init() -> ();
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub level: String,
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

pub fn init() {
    unsafe { b1Init() };
}

// 获取地图id
pub fn game_state() -> GameState {
    let info = unsafe { getPlayerInfo() };

    let level = String::from_utf8_lossy(&info.level_name)
        .trim_matches(char::from(0))
        .to_string();

    let angle = info.angle + 90.0;
    let angle = if angle < 0.0 { angle + 360.0 } else { angle };

    // 创建新的状态
    let current_state = GameState {
        playing: info.is_local_view_target == 1,
        level: level.clone(),
        angle,
        x: info.x,
        y: info.y,
        z: info.z,
    };

    // 检查是否需要使用上一次的状态
    let mut last_state = LAST_STATE.lock().unwrap();
    let final_state = if !level.is_empty() && info.x == 0.0 && info.y == 0.0 && info.z == 0.0 {
        // 如果有上一次的状态，使用它
        last_state.clone().unwrap_or(current_state.clone())
    } else {
        // 否则使用当前状态
        current_state.clone()
    };

    // 更新保存的状态
    *last_state = Some(final_state.clone());

    final_state
}
