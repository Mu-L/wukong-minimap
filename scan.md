loadPluginDlls : pluginsPath=[CSharpLoader\Plugins]
thread '<unnamed>' panicked at src\tools.rs:68:30:
called `Result::unwrap()` on an `Err` value: Error { code: HRESULT(0x80070005), message: "拒绝访问。" }
stack backtrace:
   0:     0x7ffc23f5be81 - DllMain
   1:     0x7ffc23f707d9 - DllMain
   2:     0x7ffc23f58c57 - DllMain
   3:     0x7ffc23f5dc29 - DllMain
   4:     0x7ffc23f5d7ac - DllMain
   5:     0x7ffc23f5e532 - DllMain
   6:     0x7ffc23f5e376 - DllMain
   7:     0x7ffc23f5c5cf - DllMain
   8:     0x7ffc23f5df86 - DllMain
   9:     0x7ffc241acf74 - LM_FindSymbolAddressDemangled
  10:     0x7ffc241ad390 - LM_FindSymbolAddressDemangled
  11:     0x7ffc23ec900d - <unknown>
  12:     0x7ffc23ed20b7 - DllMain
  13:     0x7ffc241a372e - LM_FindSymbolAddressDemangled
  14:     0x7ffd44618b8f - RtlActivateActivationContextUnsafeFast
  15:     0x7ffd4465d52d - RtlEnumerateEntryHashTable
  16:     0x7ffd4465d2de - RtlEnumerateEntryHashTable
  17:     0x7ffd4461db1d - RtlCopyUnicodeString
  18:     0x7ffd44618e30 - RtlImageRvaToSection
  19:     0x7ffd446090cc - RtlUnicodeToCustomCPN
  20:     0x7ffd4461a74a - LdrLoadDll
  21:     0x7ffd4191b632 - LoadLibraryExW
  22:     0x7ffcf78f1723 - OverlayHookD3D3
  23:     0x7ffcf78f1bfb - OverlayHookD3D3
  24:     0x7ffcf91f854e - HidP_UsageListDifference
  25:     0x7ffcf91f1f8f - <unknown>
  26:     0x7ffd4350259d - BaseThreadInitThunk
  27:     0x7ffd4464af38 - RtlUserThreadStart
thread '<unnamed>' panicked at core\src\panicking.rs:221:5:
panic in a function that cannot unwind
stack backtrace:
   0:     0x7ffc23f5be81 - DllMain
   1:     0x7ffc23f707d9 - DllMain
   2:     0x7ffc23f58c57 - DllMain
   3:     0x7ffc23f5dc29 - DllMain
   4:     0x7ffc23f5d7ac - DllMain
   5:     0x7ffc23f5e532 - DllMain
   6:     0x7ffc23f5e33f - DllMain
   7:     0x7ffc23f5c5cf - DllMain
   8:     0x7ffc23f5df86 - DllMain
   9:     0x7ffc241acfbd - LM_FindSymbolAddressDemangled
  10:     0x7ffc241ad063 - LM_FindSymbolAddressDemangled
  11:     0x7ffc241ad0fb - LM_FindSymbolAddressDemangled
  12:     0x7ffc23ed2b01 - DllMain
  13:     0x7ffd24abf730 - _CxxFrameHandler3
  14:     0x7ffd24ab33d8 - is_exception_typeof
  15:     0x7ffd44694ac6 - RtlCaptureContext2
  16:     0x7ffc23ed20b7 - DllMain
  17:     0x7ffc241a372e - LM_FindSymbolAddressDemangled
  18:     0x7ffd44618b8f - RtlActivateActivationContextUnsafeFast
  19:     0x7ffd4465d52d - RtlEnumerateEntryHashTable
  20:     0x7ffd4465d2de - RtlEnumerateEntryHashTable
  21:     0x7ffd4461db1d - RtlCopyUnicodeString
  22:     0x7ffd44618e30 - RtlImageRvaToSection
  23:     0x7ffd446090cc - RtlUnicodeToCustomCPN
  24:     0x7ffd4461a74a - LdrLoadDll
  25:     0x7ffd4191b632 - LoadLibraryExW
  26:     0x7ffcf78f1723 - OverlayHookD3D3
  27:     0x7ffcf78f1bfb - OverlayHookD3D3
  28:     0x7ffcf91f854e - HidP_UsageListDifference
  29:     0x7ffcf91f1f8f - <unknown>
  30:     0x7ffd4350259d - BaseThreadInitThunk
  31:     0x7ffd4464af38 - RtlUserThreadStart
thread caused non-unwinding panic. aborting.
