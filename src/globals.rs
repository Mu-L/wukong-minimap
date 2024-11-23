memory_rs::scoped_no_mangle! {
    g_map_address: usize = 0x0;
    g_map_back_address: usize = 0x0;
    g_pos_address: usize = 0x0;
    g_pos_back_address: usize = 0x0;
    g_pause_address: usize = 0x0;
    g_pause_back_address: usize = 0x0;
}

extern "C" {
    pub static asm_map_hook: u8;
    pub static asm_pos_hook: u8;
    pub static asm_pause_hook: u8;
}
