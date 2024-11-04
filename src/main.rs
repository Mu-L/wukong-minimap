use hudhook::inject::Process;

fn main() {
    Process::by_name("b1-Win64-Shipping.exe")
        .unwrap()
        .inject("minimap.dll".into())
        .unwrap();
    // Process::by_name("eldenring.exe").unwrap().inject("black_myth_map.dll".into()).unwrap();
}
