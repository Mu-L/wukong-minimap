#include "stdafx.h"
#include "helper.hpp"

#include <safetyhook.hpp>

// SDK
#include "SDK/Engine_classes.hpp"

// 添加头文件引用
#include "exports.h"

HMODULE baseModule = GetModuleHandle(NULL);



//场景ID
//13 = 序章云端_MGD
//10 = 黑风山
//11 = 隐·旧观音禅院
//12 = 黑风山 - 尺木间
//20 = 黄风岭
//25 = 隐·斯哈里国
//24 = 黄风岭 - 藏龙洞
//30 = 小西天
//62 = 隐·梅山
//92 = 小西天 - 浮屠塔
//40 = 盘丝岭
//80 = 隐·紫云山
//50 = 火焰山
//70 = 隐·壁水洞
//98 = 花果山
//61 = 石卵
//31 = 如意画轴 - 六六村
//
//103 = 除错房间
//4444 = 测试空房间
//4445 = 战斗记录地图
//1 = *开始界面 *
//3 = ***FTT_RZD_01
//4 = map_derek_test
//5 = map_hatum_test
//6 = map_shep_test
//7 = SetConfig_P
//9 = ***花果山顶HGS01_PersistentLevel
//
//21 = ***HFM_XiaoXuMi_01
//22 = ***HFM_HouHu_Cave01
//23 = ***Online_SHLG1_Persist
//
//51 = ***HYS_RYGD_PersistentLevel
//60 = ***EndA01_PersistentLevel
//90 = ***FTT01_persistentlevel
//91 = ***BDM
//97 = ***两界山LJS_persistentlevel
//99 = ***神之猎场SZLC01_persistentlevel
//100 = map_rebirth_test
//101 = CharlieTestOnline
//102 = map_cd9_fx_wlg
//104 = ONLINE_test
//105 = ONLINE_guansi_test01
//106 = Demo_Battle_Rongda
//107 = map_sybil_test
//108 = cd9测试
//109 = Demo_Battle_Final
//110 = Demo_Battle_Feiluo
//111 = ***石敢当SGD01_persistentlevel
//112 = BYS_test
//113 = Demo_jasonwu
//1000 = Demo_Battle_scene00
//3615 = Demo_Battle_Ziheng
//3616 = map_chris_test_ex
//3617 = ***Demo_Battle_Jeffrey
//3618 = ***DebugRoom_Travel_Persistent
//3619 = DebugRoom_collection
//3620 = ***DebugRoom_MoveMode
//3621 = map_jzptest_3
//3622 = ***Test_SJJ
//3623 = ***LHC_party_map
//3624 = Boss01_test_map
//3625 = Boss02_test_map
//3626 = Boss03_test_map
//3627 = map_migge_Collection
//3630 = ***GamePlayTraceTest_Battlet
//3631 = guansi_tianbing
//3632 = fjay_test_scene_mini
//3633 = ***test_XF
//3634 = Demo_Battle_Transform
//3635 = Demo_Battle_VigorSkill
//3636 = ***guansi_tianjiang

// 添加关卡ID映射
std::unordered_map<std::string, int32_t> levelIdMap = {
    //31=如意画轴-六六村
    { "LYS_paintingworld_01", 31 }, // 
    { "HFS01_PersistentLevel", 10 },
    { "HFS01_Old_GYCY_YKX_PersistentLevel", 11 },
    { "HFM02_PersistentLevel", 20 },
    { "HFM_DuJiaoXian_Persist", 25 },
    { "LYS_PersistentLevel", 30 },
    { "PSD_PersistentLevel", 40 },
    { "ZYS01_persistentlevel", 80 },
    { "HYS_PersistentLevel", 50 },
    { "BYS_persistentlevel", 98 },
    { "BSD01_Guide", 70 }
};

extern "C" __declspec(dllexport) void InitOffsets()
{
    // GObjects
    uint8_t* GObjectsScanResult = Memory::PatternScan(baseModule, "48 8B ?? ?? ?? ?? ?? 48 8B ?? ?? 48 8D ?? ?? EB ?? 33 ?? 8B ?? ?? C1 ??");
	if (GObjectsScanResult) {
		//spdlog::info("Offsets: GObjects: Address is {:s}+{:x}", sExeName.c_str(), (uintptr_t)GObjectsScanResult - (uintptr_t)baseModule);
        uintptr_t GObjectsAddr = Memory::GetAbsolute((uintptr_t)GObjectsScanResult + 0x3);
        SDK::Offsets::GObjects = (uintptr_t)GObjectsAddr - (uintptr_t)baseModule;
        //spdlog::info("Offsets: GObjects: Offset: {:x}", SDK::Offsets::GObjects);
    }
    else if (!GObjectsScanResult) {
        //spdlog::error("Offsets: GObjects: Pattern scan failed.");
    }

    // AppendString
    uint8_t* AppendStringScanResult = Memory::PatternScan(baseModule, "48 89 ?? ?? ?? 57 48 83 ?? ?? 80 3D ?? ?? ?? ?? 00 48 8B ?? 48 8B ?? 74 ?? 4C 8D ?? ?? ?? ?? ?? EB ?? 48 8D ?? ?? ?? ?? ?? E8 ?? ?? ?? ?? 4C ?? ?? C6 ?? ?? ?? ?? ?? 01 8B ?? 8B ?? 0F ?? ?? C1 ?? 10 89 ?? ?? ?? 89 ?? ?? ?? 48 8B ?? ?? ?? 48 ?? ?? ?? 8D ?? ?? 49 ?? ?? ?? ?? 48 8B ?? E8 ?? ?? ?? ?? 83 ?? ?? 00");
    if (AppendStringScanResult) {
        //spdlog::info("Offsets: AppendString: Address is {:s}+{:x}", sExeName.c_str(), (uintptr_t)AppendStringScanResult - (uintptr_t)baseModule);
        SDK::Offsets::AppendString = (uintptr_t)AppendStringScanResult - (uintptr_t)baseModule;
        //spdlog::info("Offsets: AppendString: Offset: {:x}", SDK::Offsets::AppendString);
    }
    else if (!AppendStringScanResult) {
        //spdlog::error("Offsets: AppendString: Pattern scan failed.");
    }

    // ProcessEvent
    uint8_t* ProcessEventScanResult = Memory::PatternScan(baseModule, "40 ?? 56 57 41 ?? 41 ?? 41 ?? 41 ?? 48 ?? ?? ?? ?? ?? ?? 48 8D ?? ?? ?? 48 89 ?? ?? ?? ?? ?? 48 8B ?? ?? ?? ?? ?? 48 ?? ?? 48 89 ?? ?? ?? ?? ?? 4D ?? ?? 48 ?? ?? 4C ?? ?? 48 ?? ??");
    if (ProcessEventScanResult) {
        //spdlog::info("Offsets: ProcessEvent: Address is {:s}+{:x}", sExeName.c_str(), (uintptr_t)ProcessEventScanResult - (uintptr_t)baseModule);
        SDK::Offsets::ProcessEvent = (uintptr_t)ProcessEventScanResult - (uintptr_t)baseModule;
        //spdlog::info("Offsets: ProcessEvent: Offset: {:x}", SDK::Offsets::ProcessEvent);
    }
    else if (!ProcessEventScanResult) {
        //spdlog::error("Offsets: ProcessEvent: Pattern scan failed.");
    }
}

// 修改getGameInfo方法
extern "C" __declspec(dllexport) GameInfo GetGameInfo() {
    GameInfo info = {
        {0},           // levelName 数组初始化为0
        -1.0f,        // x
        -1.0f,        // y
        -1.0f,        // z
        0.0f,         // angle
        0             // bIsLocalViewTarget
    };
    
    SDK::UWorld* World = SDK::UWorld::GetWorld();
    if (!World) return info;

    // 检查GameMode
    SDK::AGameModeBase* GameMode = World->AuthorityGameMode;
    if (!GameMode) return info;

    // 检查GameState
    SDK::AGameStateBase* GameState = World->GameState;
    if (!GameState) return info;

    // 检查PlayerController
    SDK::APlayerController* PlayerController = World->OwningGameInstance->LocalPlayers[0]->PlayerController;
    if (!PlayerController) return info;

    // 检查是否有有效的Pawn
    if (!PlayerController->Pawn) return info;

    // 获取当前关卡名称
    std::string LevelName = World->PersistentLevel->Outer->GetName();
    strncpy_s(reinterpret_cast<char*>(info.levelName), sizeof(info.levelName), LevelName.c_str(), sizeof(info.levelName) - 1);

    // 设置地图ID
    // auto it = levelIdMap.find(LevelName);
    // info.mapid = (it != levelIdMap.end()) ? it->second : 0;

    // 获取位置和角度
    SDK::FVector Location = PlayerController->Pawn->K2_GetActorLocation();
    SDK::FRotator Rotator = PlayerController->Pawn->K2_GetActorRotation();
    
    info.x = Location.X;
    info.y = Location.Y;
    info.z = Location.Z;
    info.angle = Rotator.Yaw;
    
    // 综合判断游戏状态
    bool isPlayable = true;
    

	// 1. 检查是否在加载关卡
    //  bIsLocalViewTarget 是 uint8;
	//spdlog::info("Pawn: bIsLocalViewTarget: {} ", (int)PlayerController->Pawn->bIsLocalViewTarget);


	//spdlog::info("Pawn: IsPawnControlled: {} ", PlayerController->Pawn->IsPawnControlled());


	//spdlog::info("InputComponent: bIsActive: {} ", (int)PlayerController->InputComponent->bIsActive);
	// 1. 检查是否在加载关卡
    
	//spdlog::info("IsGamePaused: {} bPauseable: {}", SDK::UGameplayStatics::IsGamePaused(World));

    //info.playing = !PlayerController->bShowMouseCursor;

	info.bIsLocalViewTarget = PlayerController->Pawn->bIsLocalViewTarget;
    return info;
}

// 修改MonitorPlayerInfo方法来使用新的结构体
//void MonitorPlayerInfo() {
//    while (true) {
//        GameInfo info = GetGameInfo();
//        
//       /* spdlog::info("地图ID: {}, 坐标: X={:.2f}, Y={:.2f}, Z={:.2f}, 角度: {:.2f}, 可玩状态: {}", 
//            info.mapid, info.x, info.y, info.z, info.angle, 
//            info.playing);*/
//
//        std::this_thread::sleep_for(std::chrono::seconds(1));
//    }
//}


//BOOL APIENTRY DllMain(HMODULE hModule,
//    DWORD  ul_reason_for_call,
//    LPVOID lpReserved
//    )
//{
//    switch (ul_reason_for_call)
//    {
//    case DLL_PROCESS_ATTACH:
//    {
//        InitOffsets();
//        break;
//    }
//    case DLL_THREAD_ATTACH:
//    case DLL_THREAD_DETACH:
//    case DLL_PROCESS_DETACH:
//        break;
//    }
//    return TRUE;
//}