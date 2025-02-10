#include <vector>
#include "MinHook.h"

#include "directhook.h"
#include "method_table.h"

#include "D3D12/d3d12hook.h"

namespace directhook
{
	namespace
	{
		bool gInitialized = false;
		MethodTable gMethodTable;
	}

	DH_Status DH_Initialize()
	{
		DH_Status status = DH_Status::Error_NoGfxApi;

		status = d3d12::Initialize(gMethodTable);
		if (status != DH_Status::Success)
		{
			return status;
		}

		if (status == DH_Status::Error_NoGfxApi)
		{
			return status;
		}
		
		MH_STATUS mhStatus = MH_Initialize();
		if (mhStatus != MH_OK)
		{
			return DH_Status::Error_MinHookInitFailed;
		}
		gInitialized = true;
		return DH_Status::Success;
	}

	void DH_Shutdown()
	{
		if (gInitialized)
		{
			MH_DisableHook(MH_ALL_HOOKS);
		}
	}

	DH_Status Hook(Uint16 index, void** original, void* function)
	{
		DH_ASSERT(original != nullptr && function != nullptr);

		if (gInitialized)
		{
			void* target = gMethodTable[index];
			if (MH_CreateHook(target, function, original) != MH_OK)
			{
				return DH_Status::Error_MinHookFailed;
			}
			if (MH_EnableHook(target) != MH_OK)
			{
				return DH_Status::Error_MinHookEnableFailed;
			}
			return DH_Status::Success;
		}
		return DH_Status::Error_NoGfxApi;
	}

	DH_Status Unhook(Uint16 index)
	{
		if (gInitialized)
		{
			MH_STATUS mhStatus = MH_DisableHook(gMethodTable[index]);
			if (mhStatus != MH_OK)
			{
				return DH_Status::Error_MinHookFailed;
			}
			return DH_Status::Success;
		}
		return DH_Status::Error_NoGfxApi;
	}

	void* GetOriginal(Uint16 index)
	{
		return gMethodTable[index];
	}

}