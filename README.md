


# 黑神话·悟空-内置实时地图

Switch language: [English](README.en.md)

该插件是本人学习 Rust 的练手项目，基于[imgui](https://github.com/ocornut/imgui) 和 [hudhook](https://github.com/veeenu/hudhook?from=jaskang)库开发。该项目为免费开源请勿用作商业用途，作者不承担任何责任。

如果可以的话 请帮我点个 star ，这对我很重要。

Nexusmods 地址: [https://www.nexusmods.com/blackmythwukong/mods/956](https://www.nexusmods.com/blackmythwukong/mods/956)

![image](./docs/images/minmap.png)

### 安装

1. 先安装依赖  [CSharpLoader﻿](https://www.nexusmods.com/blackmythwukong/mods/664)

2. 下载 Mod: [BlackMythMap](https://www.nexusmods.com/blackmythwukong/mods/956)
 
3. 将 BlackMythMap.zip 直接 解压至黑神话的安装文件夹中 (steam的安装文件夹可以通过右键黑神话 -> 管理 -> 浏览本地文件 找到)

### 按键说明

键盘：`M` 键打开/关闭大地图，`N` 开启关闭 Mod
手柄：`Back` 键打开/关闭大地图

![image](./docs/images/bigmap.png)

### 卸载

将 b1\Binaries\Win64\CSharpLoader\Mods\ 里的﻿  BlackMythMap 文件夹删除即可


### 完善地图数据

玩家在游戏中发现地图或点位信息错误，可截图反馈在 [issue](https://github.com/jaskang/jas_minimap/issues) 中，作者会及时更新

### 打赏

如果本插件对你有帮助，请作者喝杯咖啡吧

<div align="">
<img src="./docs/images/donate.jpg" alt="打赏" style="width: 200px;" />
</div>



--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


// 地图
// 11D14EE0 - 55                    - push rbp
// 11D14EE1 - 48 8B EC              - mov rbp,rsp
// 11D14EE4 - 48 83 EC 30           - sub rsp,30 { 48 }
// 11D14EE8 - 48 89 75 F8           - mov [rbp-08],rsi
// 11D14EEC - 48 8B F1              - mov rsi,rcx
// 11D14EEF - 48 B8 70A1375E01000000 - mov rax,b1-Win64-Shipping.exe+1E37A170 { (0) }
// 11D14EF9 - F7 00 01000000        - test [rax],00000001 { 1 }
// 11D14EFF - 74 12                 - je 11D14F13
// 11D14F01 - 48 8D 64 24 00        - lea rsp,[rsp+00]
// 11D14F06 - 49 BB A0E35C0600000000 - mov r11,00000000065CE3A0 { (-326416299) }
// 11D14F10 - 41 FF D3              - call r11
// 11D14F13 - 48 C7 45 F0 00000000  - mov qword ptr [rbp-10],00000000 { 0 }
// 11D14F1B - 48 8B 46 28           - mov rax,[rsi+28]
// 11D14F1F - 48 63 56 40           - movsxd  rdx,dword ptr [rsi+40] // rsi+40 是 地图id
// 11D14F23 - 48 8B C8              - mov rcx,rax
// 11D14F26 - 4C 8B C5              - mov r8,rbp
// 11D14F29 - 49 83 C0 F0           - add r8,-10
// 11D14F2D - 83 38 00              - cmp dword ptr [rax],00
