#include <windows.h>
#include "draw.h"

#include <thread>
#include <chrono>
#include "minimap.h"

void startup_thread(HMODULE poc_module)
{
	std::this_thread::sleep_for(std::chrono::seconds(10));

	Minimap::init();
	std::this_thread::sleep_for(std::chrono::seconds(10));

	DrawInit();
}

BOOL WINAPI DllMain(HMODULE handle, DWORD reason, LPVOID reserved)
{
	DisableThreadLibraryCalls(handle);
	if (reason == DLL_PROCESS_ATTACH)
	{
		CreateThread(nullptr, 0, (LPTHREAD_START_ROUTINE)startup_thread, handle, 0, nullptr);
	}

	return TRUE;
}
