#[derive(Debug, Clone, Copy)]
pub struct IconCheckbox {
    pub key: &'static str,
    pub checked: bool,
}
impl IconCheckbox {
    pub fn new(key: &'static str, checked: bool) -> Self {
        IconCheckbox { key, checked }
    }
}

pub fn get_icons() -> Vec<IconCheckbox> {
    let icons: Vec<IconCheckbox> = vec![
        IconCheckbox::new("teleport", true),
        IconCheckbox::new("start", true),
        IconCheckbox::new("end", true),
        IconCheckbox::new("pass-route", true),
        IconCheckbox::new("hidden", true),
        IconCheckbox::new("boss", true),
        IconCheckbox::new("toumu", true),
        IconCheckbox::new("renwu", true),
        IconCheckbox::new("zhixian", true),
        IconCheckbox::new("dazuo", true),
        IconCheckbox::new("baoxiang", true),
        IconCheckbox::new("bianhua", true),
        IconCheckbox::new("fabao", true),
        IconCheckbox::new("pigua", true),
        IconCheckbox::new("zhenwan", true),
        IconCheckbox::new("hulu", true),
        IconCheckbox::new("jiushi", true),
        IconCheckbox::new("xiandan", true),
        IconCheckbox::new("yongpin", true),
        IconCheckbox::new("yaopin", true),
        IconCheckbox::new("cailiao", true),
        IconCheckbox::new("jingpo", true),
        IconCheckbox::new("yaocai", true),
        IconCheckbox::new("luojia", true),
        IconCheckbox::new("sandongchong", true),
        IconCheckbox::new("lingyun", true),
    ];
    icons
}
