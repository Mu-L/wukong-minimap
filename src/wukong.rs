#![feature(naked_functions)]

use std::arch::asm;
use std::thread;
use std::time::Duration;

use hudhook::tracing::debug;
use hudhook::windows::Win32::System::LibraryLoader::GetModuleHandleA;
use ilhook::x64::{CallbackOption, HookFlags, HookType, Hooker, Registers};
use minhook::MinHook;

use crate::memedit::PointerChain;
use crate::tools::{load_json, point_inside, vector_to_angle};
use mem_rs::{helpers, process::Process};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy)]
pub struct MemPosition {
    pub angle_x: f64,
    pub angle_y: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
    pub name: String,
    pub category: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaInfo {
    pub id: i32,
    pub map: i32,
    pub code: String,
    pub name: String,
    pub image: String,
    pub range_x: [f32; 2],
    pub range_y: [f32; 2],
    pub range_z: [f32; 2],
    pub range_side: Option<[f32; 4]>,
    pub points: Vec<Icon>,
}

impl AreaInfo {
    pub fn width(&self) -> f32 {
        self.range_x[1] - self.range_x[0]
    }
    pub fn height(&self) -> f32 {
        self.range_y[1] - self.range_y[0]
    }
}

#[derive(Debug, Clone)]
pub struct MiniMap {
    pub key: u32,
    // 相对于地图的坐标百分比
    pub x: f32,
    // 相对于地图的坐标百分比
    pub y: f32,
    pub angle: f32,
    pub area: AreaInfo,
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}

pub struct Wukong {
    playing_chain: PointerChain<u32>,
    scene_chain: PointerChain<u32>,
    position_chain: PointerChain<MemPosition>,
    pub areas: Vec<AreaInfo>,
    pub scene: u32,
    pub playing: bool,
    pub position: Position,
    pub area: Option<AreaInfo>,
    pub minimap: Option<MiniMap>,
}

static mut map_id: u64 = 0;
static mut map_jmp_back: u64 = 0;

impl Wukong {
    pub fn new() -> Self {
        let base_address = unsafe { GetModuleHandleA(None).unwrap() }.0 as usize;

        let areas: Vec<AreaInfo> = load_json("areas.json");
        // 125E8130
        // 19085B89B - 89 B7 E4000000        - mov [rdi+000000E4],esi
        // 19085B8A1 - 48 8B CF              - mov rcx,rdi
        // 19085B8A4 - 66 90                 - nop 2
        // 19085B8A6 - 49 BB E03DF88D01000000 - mov r11,000000018DF83DE0 { (-326416299) }
        // 19085B8B0 - 41 FF D3              - call r11
        // 19085B8B3 - 85 C0                 - test eax,eax
        // 19085B8B5 - 0F85 10010000         - jne 19085B9CB

        // 464547F0 + E4
        // qword ptr ss:[rsp+70]=[00000039398FB200 &"85 C0 0F 85 27 00 00 00 B8 01 00 00 00"]=0000014947FA1788 "85 C0 0F 85 27 00 00 00 B8 01 00 00 00"

        // 1FEA13D47 - 00 70 AB              - add [rax-55],dh
        // 1FEA13D4A - FF 44 02 00           - inc [rdx+rax+00]
        // 1FEA13D4E - 00 00                 - add [rax],al
        // 1FEA13D50 - 90                    - nop
        // 1FEA13D51 - AB                    - stosd
        // 1FEA13D52 - FF 44 02 00           - inc [rdx+rax+00]
        // 1FEA13D56 - 00 00                 - add [rax],al
        // 1FEA13D58 - B0 AB                 - mov al,-55 { 171 }
        // 1FEA13D5A - FF 44 02 00           - inc [rdx+rax+00]
        // 1FEA13D5E - 00 00                 - add [rax],al
        // 1FEA13D60 - 3F                    - db 3F
        // 1FEA13D61 - 00 00                 - add [rax],al
        // 1FEA13D63 - 00 3E                 - add [rsi],bh
        // 1FEA13D65 - AB                    - stosd
        // 1FEA13D66 - 04 00                 - add al,00 { 0 }
        // 1FEA13D68 - 35 AB040000           - xor eax,000004AB { 1195 }
        // 1FEA13D6D - 00 00                 - add [rax],al
        // 1FEA13D6F - 00 00                 - add [rax],al
        // 1FEA13D71 - 00 00                 - add [rax],al
        // 1FEA13D73 - 00 62 00              - add [rdx+00],ah
        // 1FEA13D76 - 00 00                 - add [rax],al
        // 1FEA13D78 - 0A 00                 - or al,[rax] 这里可以读取到地图的 status

        // loadPluginDlls : pluginsPath=[CSharpLoader\Plugins]
        // loadPluginDlls : load CSharpLoader\Plugins\jas_minimap.dll
        // CSharpLoader enableJit: 0
        // CSharpLoader wait for init.
        // map lp_address: 8B47FFFEC0E4B789

        let mut b1 = Process::new("b1-Win64-Shipping.exe");

        b1.refresh().unwrap();
        let pointer = b1
            .scan_abs("Error message", "89 B7 E4 00 00 00 48 8B CF", 0, vec![])
            .unwrap();
        let lp_address = pointer.get_base_address();
        // 十六进制
        println!("map lp_address: {:X}", lp_address);

        // Create a hook for the return_0 function, detouring it to return_1

        // 创建一个线程 10s 后执行 hook

        Self {
            playing_chain: PointerChain::new(&[base_address + 0x1D9EF970, 0x0, 0x30, 0x48]),
            // C8 DC07 88 01 00 00 00 00 00 00 00 00 00 00 00 D0 29 CC EA
            scene_chain: PointerChain::new(&[base_address + 0x1DB02B20, 0x8, 0x68, 0x148, 0x40]),
            position_chain: PointerChain::new(&[
                base_address + 0x1D9EDCE0,
                0x0,
                0xE0,
                0x10,
                0xA0,
                0x80,
                0x58,
                0x278,
                0x2B8,
                0x190,
                0x260,
            ]),
            areas,
            scene: 0,
            playing: false,
            position: Position::default(),
            area: None,
            minimap: None,
        }

        // TODO:设置一个定时器每隔 1000ms 调用一次地图 self.status.read(), 将获取到的地图 status 更新到 map_id
    }
    // 刷新地图信息, 返回是否需要切换地图
    pub fn refresh(&mut self) -> Option<u32> {
        self.scene = self.scene_chain.read().unwrap_or(0);
        self.playing = self.playing_chain.read().unwrap_or(0) != 0;
        self.position = match self.position_chain.read() {
            Some(men) => Position {
                x: men.x as f32,
                y: men.y as f32,
                z: men.z as f32,
                angle: vector_to_angle(men.angle_x as f32, men.angle_y as f32),
            },
            None => Position::default(),
        };

        self.area = self
            .areas
            .iter()
            .rfind(|m| {
                if self.scene >= 10
                    && m.map == self.scene as i32
                    && self.position.x >= m.range_x[0]
                    && self.position.x <= m.range_x[1]
                    && self.position.y >= m.range_y[0]
                    && self.position.y <= m.range_y[1]
                    && self.position.x != 0.0
                    && self.position.y != 0.0
                {
                    if let Some(range_side) = &m.range_side {
                        let inside = point_inside(
                            [range_side[0], range_side[1]],
                            [range_side[2], range_side[3]],
                            [self.position.x, self.position.y],
                        );
                        debug!("inside: {:?}", inside);
                        if inside {
                            return true;
                        }
                    }
                    if self.position.z >= m.range_z[0] && self.position.z <= m.range_z[1] {
                        return true;
                    }
                }
                false
            })
            .map(|m| m.clone());

        let miniapp = if let Some(area) = &self.area {
            // 将游戏坐标转换为小地图坐标
            let x = (self.position.x - area.range_x[0]) / area.width();
            let y = (self.position.y - area.range_y[0]) / area.height();

            // 小地图的 x 轴是向右的，所以需要将 x 轴翻转
            // let map_x = map.size[0] as f32 - map_x;
            // 小地图的 y 轴是向下的，所以需要将 y 轴翻转
            // let map_y = map.size[1] as f32 - map_y;

            Some(MiniMap {
                key: area.id as u32,
                x,
                y,
                angle: self.position.angle,
                area: area.clone(),
            })
        } else {
            None
        };

        // 判断小地图的 key 是否发生变化，如果发生变化则返回 true
        let ret = match (&self.minimap, &miniapp) {
            (Some(a), Some(b)) => match a.key == b.key {
                true => None,
                false => Some(b.key),
            },
            (None, Some(b)) => Some(b.key),
            (Some(_), None) => Some(0),
            _ => None,
        };
        self.minimap = miniapp;

        ret
    }

    // 转换游戏坐标转为坐标百分比
    pub fn point_to_percent(&self, point: [f32; 2]) -> [f32; 2] {
        if let Some(area) = &self.area {
            let x = (point[0] - area.range_x[0]) / area.width();
            let y = (point[1] - area.range_y[0]) / area.height();
            [x, y]
        } else {
            [0.0, 0.0]
        }
    }
}
