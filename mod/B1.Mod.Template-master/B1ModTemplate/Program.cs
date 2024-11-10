using CSharpModBase.Input;
using System.Linq.Expressions;
using System.Runtime.InteropServices;
using System.Timers;
// using HarmonyLib;

namespace CSharpExample;

public sealed class MyMod : ICSharpMod
{
    // 导入 SetMapStatus 函数
    [DllImport("jas_minimap.dll", CallingConvention = CallingConvention.StdCall)]
    public static extern void SetMapStatus(
        int mapKey,    // u32
        bool status,   // u64
        float x,        // u64
        float y,        // u64
        float z,        // u64
        float angle     // u64
    );
    // 导入 toggle 函数
    [DllImport("jas_minimap.dll", CallingConvention = CallingConvention.StdCall)]
    public static extern void toggle();

    // private readonly Harmony harmony;
    public string Name => ModName;
    public string Version => ModVersion;

    public System.Timers.Timer timer = new System.Timers.Timer(200);


    public void Init()
    {
        Console.WriteLine($"{Name} Init");
        Utils.RegisterKeyBind(Key.M, () => Toggle());
        Utils.RegisterKeyBind(ModifierKeys.Control, Key.M, Toggle);
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
        MyMod.toggle();
    }
    public void Loop()
    {
        var controller = MyUtils.GetPlayerController();
        if(controller is null){
            MyMod.SetMapStatus(0, true, 0, 0, 0, 0);
            return;
        }
        var player = MyUtils.GetControlledPawn();
        if (player is null){
            MyMod.SetMapStatus(0, true, 0, 0, 0, 0);
            return;
        }
              
        var paused = controller.ShowMouseCursor;
        var levelName = MyUtils.GetWorld().GetCurrentLevelName(true);

        var x = player.GetActorLocation().X;
        var y = player.GetActorLocation().Y;
        var z = player.GetActorLocation().Z;

        // 身躯只能左右转动
        var yaw = player.GetActorRotation().Yaw;
        Console.WriteLine($"levelName:{levelName} paused:{paused} x:{x} y:{y} z:{z} yaw:{yaw}");
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
        MyMod.SetMapStatus(mapid, paused, x, y, z, yaw);
    }
}