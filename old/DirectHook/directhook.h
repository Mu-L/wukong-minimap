#pragma once
#include <cstdint>
#include <cassert>

#define DH_ASSERT(cond) assert(cond)
#define DH_ASSERT_MSG(cond, msg) assert(cond && msg)

#include "D3D12/d3d12hook_aliases.h"
#include "D3D12/d3d12hook_indices.h"


namespace directhook
{
	using Uint8		= std::uint8_t;
	using Uint16	= std::uint16_t;
	using Uint32	= std::uint32_t;
	using Uint64	= std::uint64_t;
	using Int8		= std::int8_t;
	using Int16		= std::int16_t;
	using Int32		= std::int32_t;
	using Int64		= std::int64_t;

	enum class DH_Status : Uint8
	{
		Success,
		Error_MinHookInitFailed,
		Error_GfxApiInitFailed,
		Error_DHAlreadyInitialized,
		Error_MinHookFailed,
		Error_MinHookEnableFailed,
		Error_NoGfxApi,
	};

	DH_Status	DH_Initialize();
	void	DH_Shutdown();

	DH_Status Hook(Uint16 index, void** original, void* function);
	DH_Status Unhook(Uint16 index);
	void* GetOriginal(Uint16 index);

	template<typename FuncT>
	DH_Status Hook(Uint16 index, FuncT*& original, FuncT& function)
	{
		return Hook(index, (void**)&original, (void*)function);
	}

	template<typename FuncT>
	void SaveOriginal(Uint16 index, FuncT& F)
	{
		F = (FuncT)GetOriginal(index);
	}
}
