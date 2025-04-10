#pragma once

/*
* SDK generated by Dumper-7
*
* https://github.com/Encryqed/Dumper-7
*/

// Package: BP_Player_Wukong_PiHuoZhao_01

#include "Basic.hpp"

#include "CoreUObject_structs.hpp"


namespace SDK::Params
{

// Function BP_Player_Wukong_PiHuoZhao_01.BP_Player_Wukong_PiHuoZhao_01_C.CheckDrawSignCondition
// 0x0010 (0x0010 - 0x0000)
struct BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition final
{
public:
	class AActor*                                 HitActor;                                          // 0x0000(0x0008)(BlueprintVisible, BlueprintReadOnly, Parm, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash)
	bool                                          ReturnValue;                                       // 0x0008(0x0001)(Parm, OutParm, ZeroConstructor, ReturnParm, IsPlainOldData, NoDestructor, HasGetValueTypeHash)
	bool                                          CallFunc_BGUHasBuffByID_ReturnValue;               // 0x0009(0x0001)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash)
};
//static_assert(alignof(BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition) == 0x000008, "Wrong alignment on BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition");
//static_assert(sizeof(BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition) == 0x000010, "Wrong size on BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition, HitActor) == 0x000000, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition::HitActor' has a wrong offset!");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition, ReturnValue) == 0x000008, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition::ReturnValue' has a wrong offset!");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition, CallFunc_BGUHasBuffByID_ReturnValue) == 0x000009, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_CheckDrawSignCondition::CallFunc_BGUHasBuffByID_ReturnValue' has a wrong offset!");

// Function BP_Player_Wukong_PiHuoZhao_01.BP_Player_Wukong_PiHuoZhao_01_C.ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01
// 0x0080 (0x0080 - 0x0000)
struct BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01 final
{
public:
	int32                                         EntryPoint;                                        // 0x0000(0x0004)(BlueprintVisible, BlueprintReadOnly, Parm, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash)
	uint8                                         Pad_4[0x4];                                        // 0x0004(0x0004)(Fixing Size After Last Property [ Dumper-7 ])
	class AActor*                                 K2Node_Event_HitActor;                             // 0x0008(0x0008)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash)
	struct FTransform                             K2Node_Event_HitTransform;                         // 0x0010(0x0060)(IsPlainOldData, NoDestructor)
	class UNiagaraComponent*                      CallFunc_AddComponent_ReturnValue;                 // 0x0070(0x0008)(ZeroConstructor, InstancedReference, IsPlainOldData, NoDestructor, HasGetValueTypeHash)
};
//static_assert(alignof(BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01) == 0x000010, "Wrong alignment on BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01");
//static_assert(sizeof(BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01) == 0x000080, "Wrong size on BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01, EntryPoint) == 0x000000, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01::EntryPoint' has a wrong offset!");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01, K2Node_Event_HitActor) == 0x000008, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01::K2Node_Event_HitActor' has a wrong offset!");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01, K2Node_Event_HitTransform) == 0x000010, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01::K2Node_Event_HitTransform' has a wrong offset!");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01, CallFunc_AddComponent_ReturnValue) == 0x000070, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_ExecuteUbergraph_BP_Player_Wukong_PiHuoZhao_01::CallFunc_AddComponent_ReturnValue' has a wrong offset!");

// Function BP_Player_Wukong_PiHuoZhao_01.BP_Player_Wukong_PiHuoZhao_01_C.OnDrawSign
// 0x0070 (0x0070 - 0x0000)
struct BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign final
{
public:
	class AActor*                                 HitActor;                                          // 0x0000(0x0008)(BlueprintVisible, BlueprintReadOnly, Parm, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash)
	uint8                                         Pad_8[0x8];                                        // 0x0008(0x0008)(Fixing Size After Last Property [ Dumper-7 ])
	struct FTransform                             HitTransform;                                      // 0x0010(0x0060)(BlueprintVisible, BlueprintReadOnly, Parm, IsPlainOldData, NoDestructor)
};
//static_assert(alignof(BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign) == 0x000010, "Wrong alignment on BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign");
//static_assert(sizeof(BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign) == 0x000070, "Wrong size on BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign, HitActor) == 0x000000, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign::HitActor' has a wrong offset!");
//static_assert(offsetof(BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign, HitTransform) == 0x000010, "Member 'BP_Player_Wukong_PiHuoZhao_01_C_OnDrawSign::HitTransform' has a wrong offset!");

}

