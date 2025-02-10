# Black Myth: Wukong - Built-in Real-time Map

A minimap plugin for Black Myth: Wukong that provides real-time location tracking and navigation features. It's free~free~free~

This plugin is a personal Rust learning project developed using the [imgui](https://github.com/ocornut/imgui) and [hudhook](https://github.com/veeenu/hudhook?from=jaskang) libraries. This project is free and open source, not for commercial use, and the author assumes no responsibility.

- Download: [releases](https://github.com/jaskang/wukong-minimap/releases)
- BiliBli Demo Video：[我开发了一款黑神话悟空小地图插件，精准还原行旅图 免费开源～](https://www.bilibili.com/video/BV1FxrTYeEaA/?share_source=copy_web&vd_source=dcfc3e9cca2640bbaa21c24979c4c34b)
- GitHub: [Please help me click star](https://github.com/jaskang/wukong-minimap)
- Nexusmods: [Don't forget to recommend it](https://www.nexusmods.com/blackmythwukong/mods/956)

切换语言: [中文](README.md)

## Changelog

- v1.4
  - Support adjusting map size, `\ = ` to zoom in, `\ - ` to zoom out
- v1.3
  - Modified rendering method
  - Changed loader, optimized compatibility with ue4ss, resolved charploader conflict issues
    This version changed the plugin filename, please delete old version files jas_minimap.dll, version.dll first
- v1.2
  - Added error log output
  - Attempted to adjust rendering parameters
- v1.0
  - Updated official travel map
  - Modified rendering method

## Demo Screenshots

![alt text](./docs/demo0.png)
![alt text](./docs/demo1.png)
![alt text](./docs/demo2.png)

## Installation Instructions

Extract `wukong-minimap.zip` directly to the `b1\Binaries\Win64` folder under Black Myth's installation directory (Steam installation folder can be found by right-clicking Black Myth -> Manage -> Browse Local Files)

![alt text](./docs/install0.png)

This plugin includes the following files:

- `wukong_minimap.dll` Core plugin functionality file
- `dwmapi.dll` Loader - Loads wukong_minimap.dll by proxying system functions
- `maps` Map folder

> Old versions used `jas_minimap.dll` -> `wukong_minimap.dll` and `version.dll` -> `dwmapi.dll`

If you have other means to load `wukong_minimap.dll`, you can completely skip `dwmapi.dll`

## For CSharpLoader Users

Starting from v1.3, it's compatible with CSharpLoader without special handling. For versions before v1.3, please delete the old wukong_minimap.dll file in CSharpLoader\Plugins.

## For UE4SS Users

Since ue4ss's built-in `dwmapi.dll` intercepts system APIs which prevents the plugin from loading properly, we just use the dwmapi.dll from wukong-minimap.

## Uninstallation

Simply delete the `wukong_minimap.dll` file. For versions below v1.3, delete `jas_minimap.dll`, `version.dll`, and `assets`

## WeChat Group

<div align="center">

![alt text](./docs/wechat.png)

</div>
