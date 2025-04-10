#pragma once

/*
* SDK generated by Dumper-7
*
* https://github.com/Encryqed/Dumper-7
*/

// Package: BI_ComplexSecRoleAttr

#include "Basic.hpp"

#include "BI_ComplexSecRoleAttr_classes.hpp"
#include "BI_ComplexSecRoleAttr_parameters.hpp"


namespace SDK
{

// Function BI_ComplexSecRoleAttr.BI_ComplexSecRoleAttr_C.PreConstruct
// (BlueprintCosmetic, Event, Public, BlueprintEvent)
// Parameters:
// bool                                    IsDesignTime                                           (BlueprintVisible, BlueprintReadOnly, Parm, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash)

void UBI_ComplexSecRoleAttr_C::PreConstruct(bool IsDesignTime)
{
	static class UFunction* Func = nullptr;

	if (Func == nullptr)
		Func = Class->GetFunction("BI_ComplexSecRoleAttr_C", "PreConstruct");

	Params::BI_ComplexSecRoleAttr_C_PreConstruct Parms{};

	Parms.IsDesignTime = IsDesignTime;

	UObject::ProcessEvent(Func, &Parms);
}


// Function BI_ComplexSecRoleAttr.BI_ComplexSecRoleAttr_C.ExecuteUbergraph_BI_ComplexSecRoleAttr
// (Final, UbergraphFunction)
// Parameters:
// int32                                   EntryPoint                                             (BlueprintVisible, BlueprintReadOnly, Parm, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash)

void UBI_ComplexSecRoleAttr_C::ExecuteUbergraph_BI_ComplexSecRoleAttr(int32 EntryPoint)
{
	static class UFunction* Func = nullptr;

	if (Func == nullptr)
		Func = Class->GetFunction("BI_ComplexSecRoleAttr_C", "ExecuteUbergraph_BI_ComplexSecRoleAttr");

	Params::BI_ComplexSecRoleAttr_C_ExecuteUbergraph_BI_ComplexSecRoleAttr Parms{};

	Parms.EntryPoint = EntryPoint;

	UObject::ProcessEvent(Func, &Parms);
}


// Function BI_ComplexSecRoleAttr.BI_ComplexSecRoleAttr_C.Construct
// (BlueprintCosmetic, Event, Public, BlueprintEvent)

void UBI_ComplexSecRoleAttr_C::Construct()
{
	static class UFunction* Func = nullptr;

	if (Func == nullptr)
		Func = Class->GetFunction("BI_ComplexSecRoleAttr_C", "Construct");

	UObject::ProcessEvent(Func, nullptr);
}

}

