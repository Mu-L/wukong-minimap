#pragma once

/*
* SDK generated by Dumper-7
*
* https://github.com/Encryqed/Dumper-7
*/

// Package: SpiderNavigation

#include "Basic.hpp"

#include "CoreUObject_structs.hpp"


namespace SDK
{

// ScriptStruct SpiderNavigation.SpiderNavRelations
// 0x0010 (0x0010 - 0x0000)
struct FSpiderNavRelations final
{
public:
	TArray<int32>                                 Neighbors;                                         // 0x0000(0x0010)(ZeroConstructor, NativeAccessSpecifierPublic)
};
//static_assert(alignof(FSpiderNavRelations) == 0x000008, "Wrong alignment on FSpiderNavRelations");
//static_assert(sizeof(FSpiderNavRelations) == 0x000010, "Wrong size on FSpiderNavRelations");
//static_assert(offsetof(FSpiderNavRelations, Neighbors) == 0x000000, "Member 'FSpiderNavRelations::Neighbors' has a wrong offset!");

// ScriptStruct SpiderNavigation.SpiderNavNode
// 0x0060 (0x0060 - 0x0000)
struct FSpiderNavNode final
{
public:
	struct FVector                                Location;                                          // 0x0000(0x0018)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	struct FVector                                Normal;                                            // 0x0018(0x0018)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         Index;                                             // 0x0030(0x0004)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_34[0x2C];                                      // 0x0034(0x002C)(Fixing Struct Size After Last Property [ Dumper-7 ])
};
//static_assert(alignof(FSpiderNavNode) == 0x000008, "Wrong alignment on FSpiderNavNode");
//static_assert(sizeof(FSpiderNavNode) == 0x000060, "Wrong size on FSpiderNavNode");
//static_assert(offsetof(FSpiderNavNode, Location) == 0x000000, "Member 'FSpiderNavNode::Location' has a wrong offset!");
//static_assert(offsetof(FSpiderNavNode, Normal) == 0x000018, "Member 'FSpiderNavNode::Normal' has a wrong offset!");
//static_assert(offsetof(FSpiderNavNode, Index) == 0x000030, "Member 'FSpiderNavNode::Index' has a wrong offset!");

}

