using CSharpModBase.Input;

// using HarmonyLib;

namespace CSharpExample;

public sealed class MyMod : ICSharpMod
{
    // private readonly Harmony harmony;

    public string Name => ModName;
    public string Version => ModVersion;

    public void Init()
    {
        Console.WriteLine($"{Name} Init");
        Utils.RegisterKeyBind(Key.ENTER, () => Console.WriteLine("Enter pressed"));
        Utils.RegisterKeyBind(ModifierKeys.Control, Key.ENTER, FindPlayer);
        
        // hook
        // harmony.PatchAll();
    }

    public void DeInit()
    {
        Console.WriteLine($"{Name} DeInit");
        // harmony.UnpatchAll();
    }

    private static void FindPlayer()
    {
        Console.WriteLine("Ctrl+Enter pressed");
        var player = MyUtils.GetControlledPawn();
        
        if (player == null)
        {
            Console.WriteLine("Player not found");
        }
        else
        {
            Console.WriteLine($"Player found: {player}");
            var p = MyUtils.GetPlayerController();
            //var stat = p.GetStatId();
            var path = p.GetPathName();
            var level = p.GetLevel();

            var flags = p.GetPlayerState().GetFlags();

            var x = player.GetActorLocation().mX;
            var y = player.GetActorLocation().mY;
            var z = player.GetActorLocation().mZ;

            var hp = BGUFunctionLibraryCS.GetAttrValue(player, EBGUAttrFloat.Hp);
            var hpMax = BGUFunctionLibraryCS.GetAttrValue(player, EBGUAttrFloat.HpMax);
            Console.WriteLine($"HP: {hp}/{hpMax} path:{path} level:{level} flags:{flags} x:{x} y:{y} z:{z}");
        }
    }
}