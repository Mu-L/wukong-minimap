#pragma once

/*
* SDK generated by Dumper-7
*
* https://github.com/Encryqed/Dumper-7
*/

// Package: BP_UI_GMInfoUnit

#include "Basic.hpp"

#include "b1MinusManaged_classes.hpp"


namespace SDK
{

// WidgetBlueprintGeneratedClass BP_UI_GMInfoUnit.BP_UI_GMInfoUnit_C
// 0x0010 (0x03E8 - 0x03D8)
class UBP_UI_GMInfoUnit_C final : public UBUI_Widget
{
public:
	class UImage*                                 Left_BG;                                           // 0x03D8(0x0008)(BlueprintVisible, ExportObject, ZeroConstructor, InstancedReference, IsPlainOldData, RepSkip, NoDestructor, PersistentInstance, HasGetValueTypeHash)
	class UImage*                                 Right_BG;                                          // 0x03E0(0x0008)(BlueprintVisible, ExportObject, ZeroConstructor, InstancedReference, IsPlainOldData, RepSkip, NoDestructor, PersistentInstance, HasGetValueTypeHash)

public:
	static class UClass* StaticClass()
	{
		return StaticBPGeneratedClassImpl<"BP_UI_GMInfoUnit_C">();
	}
	static class UBP_UI_GMInfoUnit_C* GetDefaultObj()
	{
		return GetDefaultObjImpl<UBP_UI_GMInfoUnit_C>();
	}
};
//static_assert(alignof(UBP_UI_GMInfoUnit_C) == 0x000008, "Wrong alignment on UBP_UI_GMInfoUnit_C");
//static_assert(sizeof(UBP_UI_GMInfoUnit_C) == 0x0003E8, "Wrong size on UBP_UI_GMInfoUnit_C");
//static_assert(offsetof(UBP_UI_GMInfoUnit_C, Left_BG) == 0x0003D8, "Member 'UBP_UI_GMInfoUnit_C::Left_BG' has a wrong offset!");
//static_assert(offsetof(UBP_UI_GMInfoUnit_C, Right_BG) == 0x0003E0, "Member 'UBP_UI_GMInfoUnit_C::Right_BG' has a wrong offset!");

}

