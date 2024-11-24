

# Black Myth: Wukong Built-in real-time map

Switch language: [中文](./README.md)

This plugin is a learning project for me to practice Rust, developed based on imgui and hudhook libraries. This project is free and open source, please do not use it for commercial purposes, the author does not take any responsibility.

If possible, please help me give a star, this is very important to me.

Nexusmods: [https://www.nexusmods.com/blackmythwukong/mods/956](https://www.nexusmods.com/blackmythwukong/mods/956)

![image](./docs/images/minmap.png)

### Pure Installation 

1. download: [BlackMythMap](https://www.nexusmods.com/blackmythwukong/mods/956)

2. Extract BlackMythMap.zip directly to the `b1\Binaries\Win64` folder under the installation folder of Black Myth (you can find the Steam installation folder by right-clicking on Black Myth -> Manage -> Browse local files)

### Using CSharpLoader

1. download: [BlackMythMap](https://www.nexusmods.com/blackmythwukong/mods/956)

2. Extract BlackMythMap.zip directly to the `b1\Binaries\Win64\CSharpLoader\Plugins` folder under the installation folder of Black Myth, and delete the version.dll file.

3. Disable the `EnableJit` configuration of CSharpLoader.

Configuration file path: `b1\Binaries\Win64\CSharpLoader\b1cs.ini`
```ini
[Settings]
EnableJit=0 // Disable JIT compilation
```

### Key Instructions
Keyboard: Press `M` to open/close the map, press `N` to toggle HUD
Controller: Press `Left Down Arrow` to open/close the map, press `Select` to toggle HUD

![image](./docs/images/bigmap.png)

### Uninstallation 

Simply delete the BlackMythMap folder in b1\Binaries\Win64\jas_minimap.dll
