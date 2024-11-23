


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

<!-- 暂停状态 -->
b1-Win64-Shipping.exe+147990F - 49 8B D6              - mov rdx,r14
b1-Win64-Shipping.exe+1479912 - 83 38 00              - cmp dword ptr [rax],00 { 0 }
b1-Win64-Shipping.exe+1479915 - E8 00338C00           - call b1-Win64-Shipping.exe+1D3CC1A 
b1-Win64-Shipping.exe+147991A - 85 C0                 - test eax,eax
b1-Win64-Shipping.exe+147991C - 75 15                 - jne b1-Win64-Shipping.exe+1479933
b1-Win64-Shipping.exe+147991E - 49 8B 47 30           - mov rax,[r15+30] 
b1-Win64-Shipping.exe+1479922 - 48 8B C8              - mov rcx,rax // 暂停 = rax + 48
b1-Win64-Shipping.exe+1479925 - 49 8B D6              - mov rdx,r14
b1-Win64-Shipping.exe+1479928 - 45 33 C0              - xor r8d,r8d
b1-Win64-Shipping.exe+147992B - 83 38 00              - cmp dword ptr [rax],00 { 0 }
b1-Win64-Shipping.exe+147992E - E8 F1328C00           - call b1-Win64-Shipping.exe+1D3CC24 
b1-Win64-Shipping.exe+1479933 - 41 83 FE 18           - cmp r14d,18 { 24 }
b1-Win64-Shipping.exe+1479937 - 0F85 81000000         - jne b1-Win64-Shipping.exe+14799BE
b1-Win64-Shipping.exe+147993D - 41 C6 47 58 01        - mov byte ptr [r15+58],01 { 1 }
b1-Win64-Shipping.exe+1479942 - E9 77000000           - jmp b1-Win64-Shipping.exe+14799BE
b1-Win64-Shipping.exe+1479947 - 49 8B 47 38           - mov rax,[r15+38]
b1-Win64-Shipping.exe+147994B - 48 8B C8              - mov rcx,rax
b1-Win64-Shipping.exe+147994E - 49 8B D6              - mov rdx,r14



b1-Win64-Shipping.exe+19799EC - 33 C0                 - xor eax,eax
b1-Win64-Shipping.exe+19799EE - 48 0FB6 F0            - movzx rsi,al
b1-Win64-Shipping.exe+19799F2 - 33 C0                 - xor eax,eax
b1-Win64-Shipping.exe+19799F4 - 4C 0FB6 F0            - movzx r14,al
b1-Win64-Shipping.exe+19799F8 - 48 8B 45 B0           - mov rax,[rbp-50]
b1-Win64-Shipping.exe+19799FC - 48 63 40 48           - movsxd  rax,dword ptr [rax+48] // 暂停 1
b1-Win64-Shipping.exe+1979A00 - 85 C0                 - test eax,eax
b1-Win64-Shipping.exe+1979A02 - 7E 29                 - jle b1-Win64-Shipping.exe+1979A2D
b1-Win64-Shipping.exe+1979A04 - 48 8B 45 B0           - mov rax,[rbp-50]
b1-Win64-Shipping.exe+1979A08 - 48 63 40 44           - movsxd  rax,dword ptr [rax+44]
b1-Win64-Shipping.exe+1979A0C - 89 45 B8              - mov [rbp-48],eax
b1-Win64-Shipping.exe+1979A0F - B8 01000000           - mov eax,00000001 { 1 }
b1-Win64-Shipping.exe+1979A14 - 4C 0FB6 F0            - movzx r14,al
b1-Win64-Shipping.exe+1979A18 - 48 8B 45 B0           - mov rax,[rbp-50]
b1-Win64-Shipping.exe+1979A1C - 48 8B C8              - mov rcx,rax
b1-Win64-Shipping.exe+1979A1F - 48 63 49 48           - movsxd  rcx,dword ptr [rcx+48] // 暂停2
b1-Win64-Shipping.exe+1979A23 - FF C9                 - dec ecx
b1-Win64-Shipping.exe+1979A25 - 89 48 48              - mov [rax+48],ecx
b1-Win64-Shipping.exe+1979A28 - E9 55000000           - jmp b1-Win64-Shipping.exe+1979A82
b1-Win64-Shipping.exe+1979A2D - 48 8B 45 B0           - mov rax,[rbp-50]
b1-Win64-Shipping.exe+1979A31 - 4C 63 60 40           - movsxd  r12,dword ptr [rax+40]
b1-Win64-Shipping.exe+1979A35 - 49 8B C4              - mov rax,r12
b1-Win64-Shipping.exe+1979A38 - 49 63 4F 18           - movsxd  rcx,dword ptr [r15+18]
b1-Win64-Shipping.exe+1979A3C - 3B C1                 - cmp eax,ecx


<!-- 取消暂停 -->
b1-Win64-Shipping.exe+197A47C - 48 C1 E1 04           - shl rcx,04 { 4 }
b1-Win64-Shipping.exe+197A480 - 48 03 C1              - add rax,rcx
b1-Win64-Shipping.exe+197A483 - 48 83 C0 20           - add rax,20 { 32 }
b1-Win64-Shipping.exe+197A487 - 49 63 4C 24 04        - movsxd  rcx,dword ptr [r12+04]
b1-Win64-Shipping.exe+197A48C - 89 48 04              - mov [rax+04],ecx
b1-Win64-Shipping.exe+197A48F - 41 C7 04 24  FFFFFFFF - mov [r12],FFFFFFFF { -1 }
b1-Win64-Shipping.exe+197A497 - 48 8B 45 C8           - mov rax,[rbp-38]
b1-Win64-Shipping.exe+197A49B - 48 63 40 44           - movsxd  rax,dword ptr [rax+44]
b1-Win64-Shipping.exe+197A49F - 41 89 44 24 04        - mov [r12+04],eax
b1-Win64-Shipping.exe+197A4A4 - 33 C0                 - xor eax,eax
b1-Win64-Shipping.exe+197A4A6 - 85 C0                 - test eax,eax
b1-Win64-Shipping.exe+197A4A8 - 33 C0                 - xor eax,eax
b1-Win64-Shipping.exe+197A4AA - 85 C0                 - test eax,eax
b1-Win64-Shipping.exe+197A4AC - 48 8B 45 C8           - mov rax,[rbp-38]
b1-Win64-Shipping.exe+197A4B0 - 44 89 68 44           - mov [rax+44],r13d
b1-Win64-Shipping.exe+197A4B4 - 48 8B 45 C8           - mov rax,[rbp-38]
b1-Win64-Shipping.exe+197A4B8 - 48 8B C8              - mov rcx,rax
b1-Win64-Shipping.exe+197A4BB - 48 63 49 48           - movsxd  rcx,dword ptr [rcx+48] // 取消暂停
b1-Win64-Shipping.exe+197A4BF - FF C1                 - inc ecx
b1-Win64-Shipping.exe+197A4C1 - 89 48 48              - mov [rax+48],ecx
b1-Win64-Shipping.exe+197A4C4 - 48 8B 45 C8           - mov rax,[rbp-38]
b1-Win64-Shipping.exe+197A4C8 - 48 8B C8              - mov rcx,rax
b1-Win64-Shipping.exe+197A4CB - 48 63 49 4C           - movsxd  rcx,dword ptr [rcx+4C]
b1-Win64-Shipping.exe+197A4CF - FF C1                 - inc ecx
b1-Win64-Shipping.exe+197A4D1 - 89 48 4C              - mov [rax+4C],ecx
b1-Win64-Shipping.exe+197A4D4 - B8 01000000           - mov eax,00000001
b1-Win64-Shipping.exe+197A4D9 - EB 13                 - jmp b1-Win64-Shipping.exe+197A4EE
b1-Win64-Shipping.exe+197A4DB - 4D 8B F5              - mov r14,r13
b1-Win64-Shipping.exe+197A4DE - 4D 63 6C 24 04        - movsxd  r13,dword ptr [r12+04]
b1-Win64-Shipping.exe+197A4E3 - 45 85 ED              - test r13d,r13d
b1-Win64-Shipping.exe+197A4E6 - 0F8D 94FEFFFF         - jnl b1-Win64-Shipping.exe+197A380
b1-Win64-Shipping.exe+197A4EC - 33 C0                 - xor eax,eax
b1-Win64-Shipping.exe+197A4EE - 48 8B 75 D0           - mov rsi,[rbp-30]
b1-Win64-Shipping.exe+197A4F2 - 48 8B 7D D8           - mov rdi,[rbp-28]
b1-Win64-Shipping.exe+197A4F6 - 4C 8B 65 E0           - mov r12,[rbp-20]
b1-Win64-Shipping.exe+197A4FA - 4C 8B 6D E8           - mov r13,[rbp-18]
b1-Win64-Shipping.exe+197A4FE - 4C 8B 75 F0           - mov r14,[rbp-10]
b1-Win64-Shipping.exe+197A502 - 4C 8B 7D F8           - mov r15,[rbp-08]
b1-Win64-Shipping.exe+197A506 - 48 8D 65 00           - lea rsp,[rbp+00]


b1-Win64-Shipping.exe+147996E - 49 8B 47 30           - mov rax,[r15+30] { 取消暂停}
b1-Win64-Shipping.exe+1479972 - 48 8B C8              - mov rcx,rax
b1-Win64-Shipping.exe+1479975 - 49 8B D6              - mov rdx,r14
b1-Win64-Shipping.exe+1479978 - 4C 8B C5              - mov r8,rbp
b1-Win64-Shipping.exe+147997B - 49 83 C0 C8           - add r8,-38 { 200 }
b1-Win64-Shipping.exe+147997F - 83 38 00              - cmp dword ptr [rax],00 { 0 }
b1-Win64-Shipping.exe+1479982 - E8 B1328C00           - call b1-Win64-Shipping.exe+1D3CC38 { ->->b1-Win64-Shipping.exe+197A840 }
b1-Win64-Shipping.exe+1479987 - 0FB6 45 C8            - movzx eax,byte ptr [rbp-38]
b1-Win64-Shipping.exe+147998B - 85 C0                 - test eax,eax
