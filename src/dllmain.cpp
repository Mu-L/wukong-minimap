#include <windows.h>
#include "draw.h"


void startup_thread(HMODULE poc_module) {
	DrawInit();
}


BOOL WINAPI DllMain(HMODULE handle, DWORD reason, LPVOID reserved) {
	DisableThreadLibraryCalls(handle);
	if (reason == DLL_PROCESS_ATTACH) {
			CreateThread(nullptr, 0, (LPTHREAD_START_ROUTINE)startup_thread, handle, 0, nullptr);
	}

	return TRUE;
}
