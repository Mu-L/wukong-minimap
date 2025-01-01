use std::collections::HashMap;
use std::sync::OnceLock;

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

// type GetGameInfoFn = unsafe extern "C" fn() -> GameInfo;

extern "C" {
    fn GetGameInfo() -> GameInfo;
}

extern "C" {
    fn InitOffsets() -> ();
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
    pub fn init() {
        unsafe { InitOffsets() };
    }
    // 获取地图id
    pub fn game_state() -> GameState {
        let info = unsafe { GetGameInfo() };
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
}
