#pragma once

/*
* SDK generated by Dumper-7
*
* https://github.com/Encryqed/Dumper-7
*/

// Package: Arteries

#include "Basic.hpp"

#include "Engine_classes.hpp"
#include "CoreUObject_classes.hpp"
#include "Arteries_structs.hpp"


namespace SDK
{

// Class Arteries.ArteriesLibrary
// 0x0000 (0x0028 - 0x0028)
class UArteriesLibrary final : public UObject
{
public:
	static class AArteriesActor* BeginDeferredActorSpawnFromClass(const class UObject* WorldContextObject, TSubclassOf<class AArteriesActor> ActorClass, const struct FTransform& SpawnTransform);
	static class AArteriesActor* FinishSpawningActor(class AArteriesActor* Actor, bool AttachToCaller);

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"ArteriesLibrary">();
	}
	static class UArteriesLibrary* GetDefaultObj()
	{
		return GetDefaultObjImpl<UArteriesLibrary>();
	}
};
//static_assert(alignof(UArteriesLibrary) == 0x000008, "Wrong alignment on UArteriesLibrary");
//static_assert(sizeof(UArteriesLibrary) == 0x000028, "Wrong size on UArteriesLibrary");

// Class Arteries.ArteriesActor
// 0x00A0 (0x0318 - 0x0278)
class AArteriesActor : public AActor
{
public:
	FMulticastInlineDelegateProperty_             OnBuildCompleted;                                  // 0x0278(0x0010)(ZeroConstructor, InstancedReference, BlueprintAssignable, NativeAccessSpecifierPublic)
	class UArteriesObject*                        FinalObject;                                       // 0x0288(0x0008)(Edit, BlueprintVisible, ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_290[0x88];                                     // 0x0290(0x0088)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	void Build(bool bForceRebuild);
	void Increment(class FName GroupName);
	void OnBuild();
	void OnBuildCompleted__DelegateSignature(class UArteriesObject* Object, const struct FTransform& Transform);
	void WaitForCount(class FName GroupName, int32 Count, float Timeout);

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"ArteriesActor">();
	}
	static class AArteriesActor* GetDefaultObj()
	{
		return GetDefaultObjImpl<AArteriesActor>();
	}
};
//static_assert(alignof(AArteriesActor) == 0x000008, "Wrong alignment on AArteriesActor");
//static_assert(sizeof(AArteriesActor) == 0x000318, "Wrong size on AArteriesActor");
//static_assert(offsetof(AArteriesActor, OnBuildCompleted) == 0x000278, "Member 'AArteriesActor::OnBuildCompleted' has a wrong offset!");
//static_assert(offsetof(AArteriesActor, FinalObject) == 0x000288, "Member 'AArteriesActor::FinalObject' has a wrong offset!");

// Class Arteries.ArteriesSettings
// 0x0008 (0x0030 - 0x0028)
class UArteriesSettings final : public UObject
{
public:
	bool                                          bBuildWhenPropertiesChanged;                       // 0x0028(0x0001)(Edit, ZeroConstructor, Config, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
	uint8                                         Pad_29[0x7];                                       // 0x0029(0x0007)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"ArteriesSettings">();
	}
	static class UArteriesSettings* GetDefaultObj()
	{
		return GetDefaultObjImpl<UArteriesSettings>();
	}
};
//static_assert(alignof(UArteriesSettings) == 0x000008, "Wrong alignment on UArteriesSettings");
//static_assert(sizeof(UArteriesSettings) == 0x000030, "Wrong size on UArteriesSettings");
//static_assert(offsetof(UArteriesSettings, bBuildWhenPropertiesChanged) == 0x000028, "Member 'UArteriesSettings::bBuildWhenPropertiesChanged' has a wrong offset!");

// Class Arteries.ArteriesObject
// 0x0160 (0x0188 - 0x0028)
class UArteriesObject final : public UObject
{
public:
	TMap<class FName, struct FArteriesPointGroup> PointGroups;                                       // 0x0028(0x0050)(Edit, NativeAccessSpecifierPublic)
	TMap<class FName, struct FArteriesEdgeGroup>  EdgeGroups;                                        // 0x0078(0x0050)(Edit, NativeAccessSpecifierPublic)
	TMap<class FName, struct FArteriesPrimitiveGroup> PrimitiveGroups;                                   // 0x00C8(0x0050)(Edit, NativeAccessSpecifierPublic)
	TMap<class UStaticMesh*, struct FArteriesInstances> InstancesMap;                                      // 0x0118(0x0050)(Edit, NativeAccessSpecifierPublic)
	uint8                                         Pad_168[0x20];                                     // 0x0168(0x0020)(Fixing Struct Size After Last Property [ Dumper-7 ])

public:
	static class UArteriesObject* Box(const struct FVector& Origin, const struct FRotator& Rotation, const struct FVector& Size, int32 NumPointsX, int32 NumPointsY, int32 NumPointsZ);
	static class UArteriesObject* Circle(const struct FVector& Origin, const struct FRotator& Rotation, const struct FVector2D& Radius, int32 NumPoints_0);
	static struct FTransform Flatten(const struct FTransform& Transform_0);
	static class UArteriesObject* Grid(const struct FVector& Origin, const struct FRotator& Rotation, const struct FVector2D& Size, int32 NumPointsX, int32 NumPointsY);
	static class UArteriesObject* Line(const struct FVector& Origin, const struct FVector& Direction, float Length, int32 NumPoints_0);
	static class UArteriesObject* Merge(class UArteriesObject* Obj0, class UArteriesObject* Obj1, class UArteriesObject* Obj2, class UArteriesObject* Obj3, class UArteriesObject* Obj4);
	static class UArteriesObject* MergeArray(const TArray<class UArteriesObject*>& Objs);
	static class UArteriesObject* New();
	static class UArteriesObject* Sphere(const struct FVector& Origin, const struct FRotator& Rotation, const struct FVector& Radius, int32 Rows, int32 Columns);
	static class UArteriesObject* Torus(const struct FVector& Origin, const struct FRotator& Rotation, const struct FVector2D& Radius, int32 Rows, int32 Columns);
	static class UArteriesObject* Tube(const struct FVector& Origin, const struct FRotator& Rotation, const struct FVector2D& Radius, float Height, int32 Rows, int32 Columns);

	class UArteriesObject* Add(class UObject* Source, const struct FTransform& Transform_0);
	class UArteriesObject* Arc(const class FString& Groups, int32 NumSegments, float Angle);
	class UArteriesObject* Blast(const class FString& Groups, const class FString& Tags, bool DeleteNonSelected);
	class UArteriesObject* BreakPoints(const class FString& Groups);
	class UArteriesObject* Bridge(class FName StartGroupName, class FName EndGroupName);
	class UArteriesObject* Carve(EArteriesAlignType StartAlign, float StartU, EArteriesAlignType EndAlign, float EndU);
	class UArteriesObject* Clean();
	class UArteriesObject* Clip(const class FString& Groups, const struct FVector& PlaneNormal, float PlaneDist, class FName PositiveGroup, class FName NegativeGroup);
	class UArteriesObject* Copy();
	class UArteriesObject* CopyAndTransform(int32 NumCopies, const struct FTransform& Transform_0);
	class UArteriesObject* CopyToPoints(const class FString& Groups, class UObject* Source, const struct FTransform& LocalTransform);
	class UArteriesObject* Divide(float Tolerance);
	class UArteriesObject* Facet(float Tolerance);
	class UArteriesObject* Fuse(const class FString& Groups, float SnapDist);
	class UArteriesObject* Ground();
	class UArteriesObject* GroupRange(const TArray<struct FArteriesGroupRange>& Groups);
	class UArteriesObject* Hole(const class FString& OuterGroups, const class FString& InnerGroups, float Tolerance);
	class UArteriesObject* MakeGrids(const class FString& Groups, float GridSize, int32 MinGrids, bool OBBOnly);
	class UArteriesObject* Measure(class FName Name_0);
	class UArteriesObject* PolyBevel(const EArteriesGroupType GroupType, const class FString& Groups, float Distance, int32 Divisions, bool bBevelSingleCurve);
	class UArteriesObject* PolyExpand(const class FString& Groups, float Offset, class FName CurveGroup, class FName SurfaceGroup, bool OutputTangents, bool DeleteSource);
	class UArteriesObject* PolyExtrude(const class FString& Groups, float Distance, float Inset, class FName FrontGroup, class FName SideGroup, bool DeleteSource);
	class UArteriesObject* Resample(bool ByEdge, float SegmentLength, int32 NumSegments, EArteriesTangentOutputType OutputType);
	class UArteriesObject* Reverse(const class FString& Groups);
	class UArteriesObject* Road(float DefaultWidth);
	class UArteriesObject* Scatter(const class FString& Groups, int32 Seed, int32 Count, float Density, int32 Iterations);
	class UArteriesObject* SetMaterial(const class FString& Groups, class UMaterialInterface* Material, const struct FVector2D& UVScale, const struct FVector2D& UVOffset, float UVRotation, bool NullOnly);
	class UArteriesObject* SortByAttribute(class FName AttrName);
	class UArteriesObject* SortRandomly(int32 Seed);
	class UArteriesObject* SubDivide(const class FString& Groups, float MinLength);
	class UArteriesObject* Sweep(class UArteriesObject* BackBones);
	class UArteriesObject* Transform(const EArteriesGroupType GroupType, const class FString& Groups, const struct FRotator& Rotation, const struct FVector& Translation, const struct FVector& Scale);
	class UArteriesObject* Triangulate();
	class UArteriesObject* Voronoi(const class FString& Groups, int32 Seed, int32 Count, float Density, int32 Iterations);

	int32 GetPointInt(class FName Key, int32 Index_0) const;
	struct FTransform GetPointTransform(int32 Index_0) const;
	int32 GetPrimitiveInt(class FName Key, int32 Index_0) const;
	int32 LastPoint() const;
	int32 LastPrimitive() const;
	int32 NumPoints() const;
	int32 NumPrimitives() const;
	bool PointInGroup(class FName GroupName, int32 Index_0) const;
	bool PrimitiveInGroup(class FName GroupName, int32 Index_0) const;

public:
	static class UClass* StaticClass()
	{
		return StaticClassImpl<"ArteriesObject">();
	}
	static class UArteriesObject* GetDefaultObj()
	{
		return GetDefaultObjImpl<UArteriesObject>();
	}
};
//static_assert(alignof(UArteriesObject) == 0x000008, "Wrong alignment on UArteriesObject");
//static_assert(sizeof(UArteriesObject) == 0x000188, "Wrong size on UArteriesObject");
//static_assert(offsetof(UArteriesObject, PointGroups) == 0x000028, "Member 'UArteriesObject::PointGroups' has a wrong offset!");
//static_assert(offsetof(UArteriesObject, EdgeGroups) == 0x000078, "Member 'UArteriesObject::EdgeGroups' has a wrong offset!");
//static_assert(offsetof(UArteriesObject, PrimitiveGroups) == 0x0000C8, "Member 'UArteriesObject::PrimitiveGroups' has a wrong offset!");
//static_assert(offsetof(UArteriesObject, InstancesMap) == 0x000118, "Member 'UArteriesObject::InstancesMap' has a wrong offset!");

}

