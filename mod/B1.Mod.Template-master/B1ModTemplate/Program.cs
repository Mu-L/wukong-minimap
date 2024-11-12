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

    // 导入 SetMapStatus 函数
    [DllImport("jas_minimap.dll", CallingConvention = CallingConvention.StdCall)]
    public static extern void update(
        int level,    // u32
        bool paused,   // u64
        float x,        // u64
        float y,        // u64
        float z,        // u64
        float angle     // u64
    );
    // 导入 toggle 函数
    [DllImport("jas_minimap.dll", CallingConvention = CallingConvention.StdCall)]
    public static extern void toggle(bool val);

    // 导入 toggle 函数
    [DllImport("jas_minimap.dll", CallingConvention = CallingConvention.StdCall)]
    public static extern void open_big_map(bool val);

    // private readonly Harmony harmony;
    public string Name => ModName;
    public string Version => ModVersion;

    public System.Timers.Timer timer = new System.Timers.Timer(100);

    public bool is_enabled = true;
    public bool is_open = false;

    public void Init()
    {
        Console.WriteLine($"{Name} Init");
        Console.WriteLine($"当前目录: {Directory.GetCurrentDirectory()}");
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
        
        

        Utils.RegisterKeyBind(Key.M, Open);
        Utils.RegisterKeyBind(Key.N, Toggle);

        Utils.RegisterGamePadBind(GamePadButton.DPadLeftDown, Open);
        Utils.RegisterGamePadBind(GamePadButton.Back, Toggle);
        // 启动定时器
        timer.Elapsed += (Object source, ElapsedEventArgs e) => Loop();
        timer.AutoReset = true;
        timer.Enabled = true;
        // hook
        // harmony.PatchAll();
    }
    public void DeInit()
    {
        if (timer!= null)
        {
           timer.Dispose();
        }
       
        Console.WriteLine($"{Name} DeInit");
        // harmony.UnpatchAll();
    }
    public void Toggle()
    {
        is_enabled = !is_enabled;
        MyMod.toggle(is_enabled);
    }
    public void Open()
    {
        is_open = !is_open; 
        MyMod.open_big_map(is_open);
        var controller = MyUtils.GetPlayerController();
        if(controller is not null){
            controller.ShowMouseCursor = is_open;
        }
    }
    public void Loop()
    {
        var controller = MyUtils.GetPlayerController();
        if(controller is null){
            MyMod.update(0, true, 0, 0, 0, 0);
            return;
        }
        var player = MyUtils.GetControlledPawn();
        if (player is null){
            MyMod.update(0, true, 0, 0, 0, 0);
            return;
        }

        var IsPawnControlled = player.IsPawnControlled();
        var IsPlayerControlled = player.IsPlayerControlled();

        var IsGamePaused = MyUtils.GetWorld().IsGamePaused();


        var IsControllerInPlayState = UGSE_EngineFuncLib.IsControllerInPlayState(controller);
        var IsActive = player.InputComponent.IsActive();

        var levelName = MyUtils.GetWorld().GetCurrentLevelName(true);
        var x = player.GetActorLocation().X;
        var y = player.GetActorLocation().Y;
        var z = player.GetActorLocation().Z;

        // 身躯只能左右转动
        var yaw = player.GetActorRotation().Yaw;
        //Console.WriteLine($"levelName:{levelName} x:{x} y:{y} z:{z} yaw:{yaw}");

        Console.WriteLine($"IsPawnControlled:{IsPawnControlled} IsActive:{IsActive} IsControllerInPlayState:{IsControllerInPlayState}");
        var maps = new Dictionary<string, int>(){
            {"HFS01_PersistentLevel",10},
            {"HFM02_PersistentLevel",20},
            {"HFM_DuJiaoXian_Persist",25},
            {"LYS_PersistentLevel",30},
            {"PSD_PersistentLevel",40},
            {"ZYS01_persistentlevel",80},
            {"HYS_PersistentLevel",50},
            {"BYS_persistentlevel",98},
            {"BSD01_Guide",70}
        };
            // 从字典中获取对应的 mapid，如果不存在则返回 0
        var mapid = maps.TryGetValue(levelName, out int id) ? id : 0;
        MyMod.update(mapid, false, x, y, z, yaw);
    }
}