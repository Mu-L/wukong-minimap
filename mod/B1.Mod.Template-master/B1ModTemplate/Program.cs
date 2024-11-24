using CSharpModBase.Input;
using System.Linq.Expressions;
using System.Runtime.InteropServices;
using System.Timers;
// using HarmonyLib;

namespace CSharpExample;

public sealed class MyMod : ICSharpMod
{
    // 添加 LoadLibrary 的 P/Invoke 声明
    [DllImport("kernel32.dll")]
    private static extern IntPtr LoadLibrary(string dllToLoad);
    public string Name => ModName;
    public string Version => ModVersion;

    public void Init()
    {
        Console.WriteLine($"{Name} Init");
        Console.WriteLine($"current dir: {Directory.GetCurrentDirectory()}");
        // {current_dir}/CSharpLoader/Mods/BlackMythMap/jas_minimap.dll
        var dllPath = Path.Combine(Directory.GetCurrentDirectory(), "CSharpLoader", "Mods", "BlackMythMap", "jas_minimap.dll");
        Console.WriteLine($"dllPath: {dllPath}");
        try
        {
            // 主动加载DLL
            LoadLibrary(dllPath);
            Console.WriteLine("loaded jas_minimap.dll");
        } catch (Exception ex) { 
            Console.WriteLine($"load jas_minimap.dll err: {ex.Message}");
        }
    }
    public void DeInit()
    {
        
    }
}