
# 黑神话·悟空-内置实时地图

Switch language: [English](README.en.md)

该插件是本人学习 Rust 的练手项目，基于[imgui](https://github.com/ocornut/imgui) 和 [hudhook](https://github.com/veeenu/hudhook?from=jaskang)库开发。该项目为免费开源请勿用作商业用途，作者不承担任何责任。

如果可以的话 请帮我点个 star ，这对我很重要。

Nexusmods 地址: [https://www.nexusmods.com/blackmythwukong/mods/956](https://www.nexusmods.com/blackmythwukong/mods/956)

![image](./docs/images/minmap.png)

### 按键说明

键盘：`M` 键打开/关闭大地图，`N` 键开启/关闭 HUD
手柄：`左下箭头` 键打开/关闭大地图，`Select` 键开启/关闭 HUD

![image](./docs/images/bigmap.png)

#### 纯净安装

1. 下载 Mod: [BlackMythMap](https://www.nexusmods.com/blackmythwukong/mods/956)

2. 将 BlackMythMap.zip 直接解压至黑神话的安装文件夹下面的 `b1\Binaries\Win64` 中 (steam的安装文件夹可以通过右键黑神话 -> 管理 -> 浏览本地文件找到) 

![image](./docs/images/minfiles.png)

#### 使用 CSharpLoader 的用户

1. 下载 Mod: [BlackMythMap](https://www.nexusmods.com/blackmythwukong/mods/956)

2. 将 BlackMythMap.zip 直接解压至黑神话的安装文件夹下面的 `b1\Binaries\Win64\CSharpLoader\Plugins` 中, 并删除 version.dll 文件。

3. 禁用 CSharpLoader 的 `EnableJit` 配置。

配置文件路径：`b1\Binaries\Win64\CSharpLoader\b1cs.ini`
```ini
[Settings]
EnableJit=0 // 禁用 JIT 编译
```


### 卸载

将 b1\Binaries\Win64\jas_minimap.dll 文件删除即可


### 完善地图数据

玩家在游戏中发现地图或点位信息错误，可截图反馈在 [issue](https://github.com/jaskang/jas_minimap/issues) 中，作者会及时更新

### 打赏

如果本插件对你有帮助，请作者喝杯咖啡吧

<div align="">
<img src="./docs/images/donate.jpg" alt="打赏" style="width: 200px;" />
</div>

