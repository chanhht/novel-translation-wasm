pub struct Dictionary {
    pub vietphrase: String,
    pub names: String,
    pub hanviet: String,
    pub luatnhan: String,
    pub pronouns: String,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            vietphrase: "dicts/vietphrase.txt".to_owned(),
            names: "dicts/names.txt".to_owned(),
            hanviet: "dicts/hanviet.txt".to_owned(),
            luatnhan: "dicts/luatnhan.txt".to_owned(),
            pronouns: "dicts/pronouns.txt".to_owned(),
        }
    }
}
