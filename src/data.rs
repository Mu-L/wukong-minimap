#[derive(Debug, Clone, Copy)]
pub struct IconCheckbox {
    pub key: &'static str,
    pub name: &'static str,
    pub checked: bool,
}
impl IconCheckbox {
    pub fn new(key: &'static str, name: &'static str, checked: bool) -> Self {
        IconCheckbox { key, name, checked }
    }
}

pub fn get_icons() -> Vec<IconCheckbox> {
    let icons: Vec<IconCheckbox> = vec![
        IconCheckbox::new("teleport", "传送点", true),
        IconCheckbox::new("start", "入口", true),
        IconCheckbox::new("end", "出口", true),
        IconCheckbox::new("pass-route", "捷径", true),
        IconCheckbox::new("hidden", "隐藏点", true),
        IconCheckbox::new("boss", "妖王", true),
        IconCheckbox::new("toumu", "头目", true),
        IconCheckbox::new("renwu", "人物", true),
        IconCheckbox::new("zhixian", "支线", true),
        IconCheckbox::new("dazuo", "打坐", true),
        IconCheckbox::new("baoxiang", "宝箱", true),
        IconCheckbox::new("bianhua", "变化", true),
        IconCheckbox::new("fabao", "法宝", true),
        IconCheckbox::new("pigua", "披挂", true),
        IconCheckbox::new("zhenwan", "珍玩", true),
        IconCheckbox::new("hulu", "葫芦", true),
        IconCheckbox::new("jiushi", "酒食", true),
        IconCheckbox::new("xiandan", "仙丹", false),
        IconCheckbox::new("yongpin", "用品", false),
        IconCheckbox::new("yaopin", "药品", false),
        IconCheckbox::new("cailiao", "材料", false),
        IconCheckbox::new("jingpo", "精魄", false),
        IconCheckbox::new("yaocai", "药材", false),
        IconCheckbox::new("luojia", "落伽香藤", false),
        IconCheckbox::new("sandongchong", "三冬虫", false),
        IconCheckbox::new("lingyun", "灵蕴", false),
    ];
    icons
}
