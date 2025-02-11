#include "stdafx.h"
#include "helper.hpp"
#include "SDK/SDK/b1MinusManaged_classes.hpp"

#include "b1sdk.h"

extern "C" __declspec(dllexport) PlayerInfo getPlayerInfo()
{
	PlayerInfo info = {
			-1.0f, // x
			-1.0f, // y
			-1.0f, // z
			0.0f,	 // angle
			0,		 // bIsLocalViewTarget
			""};

	SDK::UWorld *World = SDK::UWorld::GetWorld();
	if (!World)
	{
		return info;
	}
	SDK::UGameplayStatics *GameplayStatics = SDK::UGameplayStatics::GetDefaultObj();
	if (!GameplayStatics)
	{
		return info;
	}

	SDK::ACharacter *playerCharacter = SDK::UBGUFunctionLibrary::GetPlayerCharacter(World);
	if (!playerCharacter)
	{
		return info;
	}

	// 获取当前关卡名称
	std::string currentLevelName(GameplayStatics->GetCurrentLevelName(World, false).ToString());
	strncpy_s(info.level, sizeof(info.level), currentLevelName.c_str(), _TRUNCATE);
	// 获取位置和角度
	SDK::FVector Location = playerCharacter->K2_GetActorLocation();
	SDK::FRotator Rotator = playerCharacter->K2_GetActorRotation();

	info.x = Location.X;
	info.y = Location.Y;
	info.z = Location.Z;

	info.angle = Rotator.Yaw;

	info.bIsLocalViewTarget = playerCharacter->bIsLocalViewTarget;
	return info;
}

extern "C" __declspec(dllexport) void b1Init()
{
	HMODULE baseModule = GetModuleHandle(NULL);
	// GObjects
	uint8_t *GObjectsScanResult = Memory::PatternScan(baseModule, "48 8B ?? ?? ?? ?? ?? 48 8B ?? ?? 48 8D ?? ?? EB ?? 33 ?? 8B ?? ?? C1 ??");
	if (GObjectsScanResult)
	{
		uintptr_t GObjectsAddr = Memory::GetAbsolute((uintptr_t)GObjectsScanResult + 0x3);
		SDK::Offsets::GObjects = (uintptr_t)GObjectsAddr - (uintptr_t)baseModule;
		printf_s("GObjects: 0x%llx\n", SDK::Offsets::GObjects);
	}
	else if (!GObjectsScanResult)
	{
		printf_s("Offsets: GObjects: Pattern scan failed.");
	}

	// AppendString
	uint8_t *AppendStringScanResult = Memory::PatternScan(baseModule, "48 89 ?? ?? ?? 57 48 83 ?? ?? 80 3D ?? ?? ?? ?? 00 48 8B ?? 48 8B ?? 74 ?? 4C 8D ?? ?? ?? ?? ?? EB ?? 48 8D ?? ?? ?? ?? ?? E8 ?? ?? ?? ?? 4C ?? ?? C6 ?? ?? ?? ?? ?? 01 8B ?? 8B ?? 0F ?? ?? C1 ?? 10 89 ?? ?? ?? 89 ?? ?? ?? 48 8B ?? ?? ?? 48 ?? ?? ?? 8D ?? ?? 49 ?? ?? ?? ?? 48 8B ?? E8 ?? ?? ?? ?? 83 ?? ?? 00");
	if (AppendStringScanResult)
	{
		SDK::Offsets::AppendString = (uintptr_t)AppendStringScanResult - (uintptr_t)baseModule;
		printf_s("AppendString: 0x%llx\n", SDK::Offsets::AppendString);
	}
	else if (!AppendStringScanResult)
	{
		printf_s("Offsets: AppendString: Pattern scan failed.");
	}

	uint8_t *ProcessEventScanResult = Memory::PatternScan(baseModule, "40 ?? 56 57 41 ?? 41 ?? 41 ?? 41 ?? 48 ?? ?? ?? ?? ?? ?? 48 8D ?? ?? ?? 48 89 ?? ?? ?? ?? ?? 48 8B ?? ?? ?? ?? ?? 48 ?? ?? 48 89 ?? ?? ?? ?? ?? 4D ?? ?? 48 ?? ?? 4C ?? ?? 48 ?? ??");
	if (ProcessEventScanResult)
	{
		SDK::Offsets::ProcessEvent = (UC::int32)((uintptr_t)ProcessEventScanResult - (uintptr_t)baseModule);
		printf_s("ProcessEvent: 0x%llx\n", SDK::Offsets::ProcessEvent);
		// MH_STATUS ret = MH_CreateHook(reinterpret_cast<LPVOID>(ProcessEventScanResult), &process_event_hook, reinterpret_cast<void**>(&processEventOriginal));
	}
}
