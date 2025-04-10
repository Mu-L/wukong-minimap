#pragma once

/*
* SDK generated by Dumper-7
*
* https://github.com/Encryqed/Dumper-7
*/

// Package: TressFX

#include "Basic.hpp"

#include "CoreUObject_structs.hpp"
#include "CoreUObject_classes.hpp"
#include "Engine_structs.hpp"
#include "Engine_classes.hpp"
#include "TressFX_structs.hpp"


namespace SDK
{

// Class TressFX.TFXCardMeshComponent
// 0x0020 (0x0F50 - 0x0F30)
class UTFXCardMeshComponent final : public USkeletalMeshComponent
{
public:
	struct FSoftObjectPath                        CardMesh;                                          // 0x0F30(0x0018)(Edit, ZeroConstructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         VisibleQualityLevel;                               // 0x0F48(0x0004)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_F4C[0x4];                                      // 0x0F4C(0x0004)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TFXCardMeshComponent">();
	}
	static class UTFXCardMeshComponent* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTFXCardMeshComponent>();
	}
};
//static_assert(alignof(UTFXCardMeshComponent) == 0x000010, "Wrong alignment on UTFXCardMeshComponent");
//static_assert(sizeof(UTFXCardMeshComponent) == 0x000F50, "Wrong size on UTFXCardMeshComponent");
//static_assert(offsetof(UTFXCardMeshComponent, CardMesh) == 0x000F30, "Member 'UTFXCardMeshComponent::CardMesh' has a wrong offset!");
//static_assert(offsetof(UTFXCardMeshComponent, VisibleQualityLevel) == 0x000F48, "Member 'UTFXCardMeshComponent::VisibleQualityLevel' has a wrong offset!");

// Class TressFX.TressFXAsset
// 0x0248 (0x0270 - 0x0028)
class UTressFXAsset final : public UObject
{
public:
	uint8                                         Pad_28[0x8];                                       // 0x0028(0x0008)(Fixing Size After Last Property [ Dumper-7 ])
	int32                                         NumFollowStrandsPerGuide;                          // 0x0030(0x0004)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	float                                         RootSeparationFactor;                              // 0x0034(0x0004)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	float                                         ClippingPercentage;                                // 0x0038(0x0004)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         SegmentDivisor;                                    // 0x003C(0x0004)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	float                                         HairLengthScale;                                   // 0x0040(0x0004)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         NumVerticesFromRootNoSimulation;                   // 0x0044(0x0004)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class UTressFXBoneSkinningAsset*              TressFXBoneSkinningAsset;                          // 0x0048(0x0008)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class USkeletalMesh*                          BaseSkeleton;                                      // 0x0050(0x0008)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	bool                                          bIsValid;                                          // 0x0058(0x0001)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_59[0x3];                                       // 0x0059(0x0003)(Fixing Size After Last Property [ Dumper-7 ])
	int32                                         RawGuideCount;                                     // 0x005C(0x0004)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         TotalStrandCount;                                  // 0x0060(0x0004)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         VertexCountPerStrand;                              // 0x0064(0x0004)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         TotalVertexCount;                                  // 0x0068(0x0004)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         TotalTriangleCount;                                // 0x006C(0x0004)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         NumDependHairGroups;                               // 0x0070(0x0004)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_74[0x4];                                       // 0x0074(0x0004)(Fixing Size After Last Property [ Dumper-7 ])
	struct FBoxSphereBounds                       PreCalculateBounds;                                // 0x0078(0x0038)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, NativeAccessSpecifierPublic)
	uint8                                         Pad_B0[0x8];                                       // 0x00B0(0x0008)(Fixing Size After Last Property [ Dumper-7 ])
	struct FTressFXSimulationSettings             TressFXSimulationSettings;                         // 0x00B8(0x0170)(Edit, BlueprintVisible, NativeAccessSpecifierPublic)
	TArray<struct FTressFXSimulationSettings>     TressFXSimulationSettingsArray;                    // 0x0228(0x0010)(Edit, BlueprintVisible, ZeroConstructor, NativeAccessSpecifierPublic)
	TArray<struct FAnimationTressFXSimulationSettings> AnimationSimulationSettingsMap;                    // 0x0238(0x0010)(Edit, BlueprintVisible, ZeroConstructor, NativeAccessSpecifierPublic)
	uint8                                         Pad_248[0x28];                                     // 0x0248(0x0028)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXAsset">();
	}
	static class UTressFXAsset* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXAsset>();
	}
};
//static_assert(alignof(UTressFXAsset) == 0x000008, "Wrong alignment on UTressFXAsset");
//static_assert(sizeof(UTressFXAsset) == 0x000270, "Wrong size on UTressFXAsset");
//static_assert(offsetof(UTressFXAsset, NumFollowStrandsPerGuide) == 0x000030, "Member 'UTressFXAsset::NumFollowStrandsPerGuide' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, RootSeparationFactor) == 0x000034, "Member 'UTressFXAsset::RootSeparationFactor' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, ClippingPercentage) == 0x000038, "Member 'UTressFXAsset::ClippingPercentage' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, SegmentDivisor) == 0x00003C, "Member 'UTressFXAsset::SegmentDivisor' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, HairLengthScale) == 0x000040, "Member 'UTressFXAsset::HairLengthScale' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, NumVerticesFromRootNoSimulation) == 0x000044, "Member 'UTressFXAsset::NumVerticesFromRootNoSimulation' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, TressFXBoneSkinningAsset) == 0x000048, "Member 'UTressFXAsset::TressFXBoneSkinningAsset' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, BaseSkeleton) == 0x000050, "Member 'UTressFXAsset::BaseSkeleton' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, bIsValid) == 0x000058, "Member 'UTressFXAsset::bIsValid' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, RawGuideCount) == 0x00005C, "Member 'UTressFXAsset::RawGuideCount' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, TotalStrandCount) == 0x000060, "Member 'UTressFXAsset::TotalStrandCount' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, VertexCountPerStrand) == 0x000064, "Member 'UTressFXAsset::VertexCountPerStrand' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, TotalVertexCount) == 0x000068, "Member 'UTressFXAsset::TotalVertexCount' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, TotalTriangleCount) == 0x00006C, "Member 'UTressFXAsset::TotalTriangleCount' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, NumDependHairGroups) == 0x000070, "Member 'UTressFXAsset::NumDependHairGroups' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, PreCalculateBounds) == 0x000078, "Member 'UTressFXAsset::PreCalculateBounds' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, TressFXSimulationSettings) == 0x0000B8, "Member 'UTressFXAsset::TressFXSimulationSettings' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, TressFXSimulationSettingsArray) == 0x000228, "Member 'UTressFXAsset::TressFXSimulationSettingsArray' has a wrong offset!");
//static_assert(offsetof(UTressFXAsset, AnimationSimulationSettingsMap) == 0x000238, "Member 'UTressFXAsset::AnimationSimulationSettingsMap' has a wrong offset!");

// Class TressFX.TressFXBindingAsset
// 0x0038 (0x0060 - 0x0028)
class UTressFXBindingAsset final : public UObject
{
public:
	uint8                                         Pad_28[0x10];                                      // 0x0028(0x0010)(Fixing Size After Last Property [ Dumper-7 ])
	class UTressFXAsset*                          TressFXAsset;                                      // 0x0038(0x0008)(Edit, BlueprintVisible, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class USkeletalMesh*                          TargetSkeletalMesh;                                // 0x0040(0x0008)(Edit, BlueprintVisible, ZeroConstructor, EditConst, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_48[0x18];                                      // 0x0048(0x0018)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXBindingAsset">();
	}
	static class UTressFXBindingAsset* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXBindingAsset>();
	}
};
//static_assert(alignof(UTressFXBindingAsset) == 0x000008, "Wrong alignment on UTressFXBindingAsset");
//static_assert(sizeof(UTressFXBindingAsset) == 0x000060, "Wrong size on UTressFXBindingAsset");
//static_assert(offsetof(UTressFXBindingAsset, TressFXAsset) == 0x000038, "Member 'UTressFXBindingAsset::TressFXAsset' has a wrong offset!");
//static_assert(offsetof(UTressFXBindingAsset, TargetSkeletalMesh) == 0x000040, "Member 'UTressFXBindingAsset::TargetSkeletalMesh' has a wrong offset!");

// Class TressFX.TressFXBoneSkinningAsset
// 0x0040 (0x0068 - 0x0028)
class UTressFXBoneSkinningAsset final : public UObject
{
public:
	uint8                                         Pad_28[0x40];                                      // 0x0028(0x0040)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXBoneSkinningAsset">();
	}
	static class UTressFXBoneSkinningAsset* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXBoneSkinningAsset>();
	}
};
//static_assert(alignof(UTressFXBoneSkinningAsset) == 0x000008, "Wrong alignment on UTressFXBoneSkinningAsset");
//static_assert(sizeof(UTressFXBoneSkinningAsset) == 0x000068, "Wrong size on UTressFXBoneSkinningAsset");

// Class TressFX.TressFXComponent
// 0x04A0 (0x0A00 - 0x0560)
class UTressFXComponent final : public UPrimitiveComponent
{
public:
	bool                                          EnableRuntimeBounds;                               // 0x0558(0x0001)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	bool                                          EnableSimulation;                                  // 0x0559(0x0001)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	bool                                          EnableCapsuleCollision;                            // 0x055A(0x0001)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_55B[0x5];                                      // 0x055B(0x0005)(Fixing Size After Last Property [ Dumper-7 ])
	class UPhysicsAsset*                          HairPhysicsAsset;                                  // 0x0560(0x0008)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, IsPlainOldData, NoDestructor, AdvancedDisplay, UObjectWrapper, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class UTressFXAsset*                          Asset;                                             // 0x0568(0x0008)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, Interp, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class UMaterialInterface*                     HairMaterial;                                      // 0x0570(0x0008)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class UMaterialInterface*                     HairMaterialLowQuality;                            // 0x0578(0x0008)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	struct FTressFXShadeSettings                  ShadeSettings;                                     // 0x0580(0x0014)(Edit, BlueprintVisible, NoDestructor, NativeAccessSpecifierPublic)
	float                                         LodScreenSize;                                     // 0x0594(0x0004)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class UTressFXBindingAsset*                   MorphTargetBindingAsset;                           // 0x0598(0x0008)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint32                                        LocalSDFIdRef;                                     // 0x05A0(0x0004)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	bool                                          bGenerateCardMesh;                                 // 0x05A4(0x0001)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_5A5[0x3];                                      // 0x05A5(0x0003)(Fixing Size After Last Property [ Dumper-7 ])
	struct FComponentReference                    CardMeshComponentReference;                        // 0x05A8(0x0028)(Edit, BlueprintVisible, NativeAccessSpecifierPublic)
	bool                                          IsInGamePlay;                                      // 0x05D0(0x0001)(ZeroConstructor, Transient, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPrivate)
	uint8                                         Pad_5D1[0x3];                                      // 0x05D1(0x0003)(Fixing Size After Last Property [ Dumper-7 ])
	float                                         SimulationDelayTime;                               // 0x05D4(0x0004)(ZeroConstructor, Transient, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPrivate)
	float                                         SimulationDelayRemain;                             // 0x05D8(0x0004)(ZeroConstructor, Transient, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPrivate)
	bool                                          InitedBounds;                                      // 0x05DC(0x0001)(ZeroConstructor, Transient, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPrivate)
	uint8                                         Pad_5DD[0x423];                                    // 0x05DD(0x0423)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	void ResetPositions();

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXComponent">();
	}
	static class UTressFXComponent* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXComponent>();
	}
};
//static_assert(alignof(UTressFXComponent) == 0x000010, "Wrong alignment on UTressFXComponent");
//static_assert(sizeof(UTressFXComponent) == 0x000A00, "Wrong size on UTressFXComponent");
//static_assert(offsetof(UTressFXComponent, EnableRuntimeBounds) == 0x000558, "Member 'UTressFXComponent::EnableRuntimeBounds' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, EnableSimulation) == 0x000559, "Member 'UTressFXComponent::EnableSimulation' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, EnableCapsuleCollision) == 0x00055A, "Member 'UTressFXComponent::EnableCapsuleCollision' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, HairPhysicsAsset) == 0x000560, "Member 'UTressFXComponent::HairPhysicsAsset' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, Asset) == 0x000568, "Member 'UTressFXComponent::Asset' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, HairMaterial) == 0x000570, "Member 'UTressFXComponent::HairMaterial' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, HairMaterialLowQuality) == 0x000578, "Member 'UTressFXComponent::HairMaterialLowQuality' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, ShadeSettings) == 0x000580, "Member 'UTressFXComponent::ShadeSettings' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, LodScreenSize) == 0x000594, "Member 'UTressFXComponent::LodScreenSize' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, MorphTargetBindingAsset) == 0x000598, "Member 'UTressFXComponent::MorphTargetBindingAsset' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, LocalSDFIdRef) == 0x0005A0, "Member 'UTressFXComponent::LocalSDFIdRef' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, bGenerateCardMesh) == 0x0005A4, "Member 'UTressFXComponent::bGenerateCardMesh' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, CardMeshComponentReference) == 0x0005A8, "Member 'UTressFXComponent::CardMeshComponentReference' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, IsInGamePlay) == 0x0005D0, "Member 'UTressFXComponent::IsInGamePlay' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, SimulationDelayTime) == 0x0005D4, "Member 'UTressFXComponent::SimulationDelayTime' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, SimulationDelayRemain) == 0x0005D8, "Member 'UTressFXComponent::SimulationDelayRemain' has a wrong offset!");
//static_assert(offsetof(UTressFXComponent, InitedBounds) == 0x0005DC, "Member 'UTressFXComponent::InitedBounds' has a wrong offset!");

// Class TressFX.TressFXMesh
// 0x0068 (0x0090 - 0x0028)
class UTressFXMesh final : public UObject
{
public:
	uint8                                         Pad_28[0x68];                                      // 0x0028(0x0068)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXMesh">();
	}
	static class UTressFXMesh* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXMesh>();
	}
};
//static_assert(alignof(UTressFXMesh) == 0x000008, "Wrong alignment on UTressFXMesh");
//static_assert(sizeof(UTressFXMesh) == 0x000090, "Wrong size on UTressFXMesh");

// Class TressFX.TressFXMeshAsset
// 0x00C0 (0x00E8 - 0x0028)
class UTressFXMeshAsset final : public UObject
{
public:
	uint8                                         Pad_28[0x8];                                       // 0x0028(0x0008)(Fixing Size After Last Property [ Dumper-7 ])
	bool                                          EnableVisualizeMesh;                               // 0x0030(0x0001)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	bool                                          EnableVisualizeMeshAABB;                           // 0x0031(0x0001)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_32[0x2];                                       // 0x0032(0x0002)(Fixing Size After Last Property [ Dumper-7 ])
	float                                         CollisionMeshBoxMargin;                            // 0x0034(0x0004)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	bool                                          EnableVisualizeSDF;                                // 0x0038(0x0001)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_39[0x3];                                       // 0x0039(0x0003)(Fixing Size After Last Property [ Dumper-7 ])
	struct FIntVector                             NumSDFCells;                                       // 0x003C(0x000C)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	int32                                         NumGridOffset;                                     // 0x0048(0x0004)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	float                                         SDFCollisionMargin;                                // 0x004C(0x0004)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_50[0x98];                                      // 0x0050(0x0098)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXMeshAsset">();
	}
	static class UTressFXMeshAsset* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXMeshAsset>();
	}
};
//static_assert(alignof(UTressFXMeshAsset) == 0x000008, "Wrong alignment on UTressFXMeshAsset");
//static_assert(sizeof(UTressFXMeshAsset) == 0x0000E8, "Wrong size on UTressFXMeshAsset");
//static_assert(offsetof(UTressFXMeshAsset, EnableVisualizeMesh) == 0x000030, "Member 'UTressFXMeshAsset::EnableVisualizeMesh' has a wrong offset!");
//static_assert(offsetof(UTressFXMeshAsset, EnableVisualizeMeshAABB) == 0x000031, "Member 'UTressFXMeshAsset::EnableVisualizeMeshAABB' has a wrong offset!");
//static_assert(offsetof(UTressFXMeshAsset, CollisionMeshBoxMargin) == 0x000034, "Member 'UTressFXMeshAsset::CollisionMeshBoxMargin' has a wrong offset!");
//static_assert(offsetof(UTressFXMeshAsset, EnableVisualizeSDF) == 0x000038, "Member 'UTressFXMeshAsset::EnableVisualizeSDF' has a wrong offset!");
//static_assert(offsetof(UTressFXMeshAsset, NumSDFCells) == 0x00003C, "Member 'UTressFXMeshAsset::NumSDFCells' has a wrong offset!");
//static_assert(offsetof(UTressFXMeshAsset, NumGridOffset) == 0x000048, "Member 'UTressFXMeshAsset::NumGridOffset' has a wrong offset!");
//static_assert(offsetof(UTressFXMeshAsset, SDFCollisionMargin) == 0x00004C, "Member 'UTressFXMeshAsset::SDFCollisionMargin' has a wrong offset!");

// Class TressFX.TressFXMeshImportOptions
// 0x0008 (0x0030 - 0x0028)
class UTressFXMeshImportOptions final : public UObject
{
public:
	class USkeletalMesh*                          Skeleton;                                          // 0x0028(0x0008)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXMeshImportOptions">();
	}
	static class UTressFXMeshImportOptions* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXMeshImportOptions>();
	}
};
//static_assert(alignof(UTressFXMeshImportOptions) == 0x000008, "Wrong alignment on UTressFXMeshImportOptions");
//static_assert(sizeof(UTressFXMeshImportOptions) == 0x000030, "Wrong size on UTressFXMeshImportOptions");
//static_assert(offsetof(UTressFXMeshImportOptions, Skeleton) == 0x000028, "Member 'UTressFXMeshImportOptions::Skeleton' has a wrong offset!");

// Class TressFX.TressFXSDFComponent
// 0x0040 (0x05C0 - 0x0580)
class UTressFXSDFComponent final : public UMeshComponent
{
public:
	uint8                                         Pad_580[0x8];                                      // 0x0580(0x0008)(Fixing Size After Last Property [ Dumper-7 ])
	bool                                          EnableSDF;                                         // 0x0588(0x0001)(Edit, BlueprintVisible, BlueprintReadOnly, ZeroConstructor, IsPlainOldData, Interp, NoDestructor, AdvancedDisplay, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_589[0x3];                                      // 0x0589(0x0003)(Fixing Size After Last Property [ Dumper-7 ])
	uint32                                        LocalSDFId;                                        // 0x058C(0x0004)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	class UTressFXMeshAsset*                      TressFXMeshAsset;                                  // 0x0590(0x0008)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, Interp, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	bool                                          HasBlendShapes;                                    // 0x0598(0x0001)(Edit, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_599[0x27];                                     // 0x0599(0x0027)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	void SetTressFXMeshAsset(class UTressFXMeshAsset* Asset);

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"TressFXSDFComponent">();
	}
	static class UTressFXSDFComponent* GetDefaultObj()
	{
		return GetDefaultObjImpl<UTressFXSDFComponent>();
	}
};
//static_assert(alignof(UTressFXSDFComponent) == 0x000010, "Wrong alignment on UTressFXSDFComponent");
//static_assert(sizeof(UTressFXSDFComponent) == 0x0005C0, "Wrong size on UTressFXSDFComponent");
//static_assert(offsetof(UTressFXSDFComponent, EnableSDF) == 0x000588, "Member 'UTressFXSDFComponent::EnableSDF' has a wrong offset!");
//static_assert(offsetof(UTressFXSDFComponent, LocalSDFId) == 0x00058C, "Member 'UTressFXSDFComponent::LocalSDFId' has a wrong offset!");
//static_assert(offsetof(UTressFXSDFComponent, TressFXMeshAsset) == 0x000590, "Member 'UTressFXSDFComponent::TressFXMeshAsset' has a wrong offset!");
//static_assert(offsetof(UTressFXSDFComponent, HasBlendShapes) == 0x000598, "Member 'UTressFXSDFComponent::HasBlendShapes' has a wrong offset!");

}

