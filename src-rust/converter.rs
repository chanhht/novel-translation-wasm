use aho_corasick::*;
use bit_vec::BitVec;
use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Converter {
    init: bool,
    luatnhan_map: HashMap<String, String>,
    vietphrase_map: HashMap<String, String>,
    hanviet_map: HashMap<String, String>,
    names_map: HashMap<String, String>,
    pronouns_map: HashMap<String, String>,

    luatnhan_pair_phrases: HashMap<String, String>,
    aho_corasick_luatnhan: Option<AhoCorasick>,
}

#[wasm_bindgen]
impl Converter {
    pub fn new() -> Converter {
        Converter {
            init: false,
            luatnhan_map: HashMap::new(),
            vietphrase_map: HashMap::new(),
            hanviet_map: HashMap::new(),
            names_map: HashMap::new(),
            pronouns_map: HashMap::new(),
            luatnhan_pair_phrases: HashMap::new(),
            aho_corasick_luatnhan: None,
        }
    }

    pub fn convert(&mut self, content: &str
    ) -> String {

        if !self.init {
            for (k, v) in &self.names_map {
                self.vietphrase_map.insert(k.to_string(), v.to_string());
            }
            for (k, v) in &self.pronouns_map {
                self.vietphrase_map.insert(k.to_string(), v.to_string());
            }
            // build luatnhan aho corasick
            let mut luatnhan_phrases = HashSet::new();
            for (k, v) in self.luatnhan_map.iter() {
                if k.is_empty() {
                    continue;
                }
                let mut ps = k.trim().splitn(2, "{0}");
                let ps1 = ps.next().unwrap().trim().to_string();
                let ps2 = ps.next().unwrap().trim().to_string();
                if !ps1.is_empty() {
                    luatnhan_phrases.insert(ps1.clone());
                } else {
                    self.luatnhan_pair_phrases.insert(format!("_{}", &ps2), v.to_string());
                }
    
                if !ps2.is_empty() {
                    luatnhan_phrases.insert(ps2.clone());
                } else {
                    self.luatnhan_pair_phrases.insert(format!("{}_", &ps1), v.to_string());
                }
    
                if !ps1.is_empty() && !ps2.is_empty() {
                    self.luatnhan_pair_phrases.insert(format!("{}_{}", ps1, ps2), v.to_string());
                }
            }
    
            self.aho_corasick_luatnhan = Some(AhoCorasickBuilder::new()
                .match_kind(MatchKind::LeftmostLongest)
                .build(luatnhan_phrases));

            self.init = true;
        }


        let mut pre_mat: Option<Match> = None;
        let mut luatnhan_pairs = HashMap::new();
        let mut luatnhan_right_edges = HashMap::new();
        let mut luatnhan_left_edges = HashMap::new();
        let mut luatnhan_right_edges_value = HashMap::new();
        let mut luatnhan_left_edges_value = HashMap::new();
        let mut luatnhan_idx_pairs = HashMap::new();
        for mat in self.aho_corasick_luatnhan.as_ref().expect("Resources missing").find_iter(content) {
            let luatnhan_phrase = content[mat.start()..mat.end()].trim();
            let start = mat.start();
            let end = mat.end();
            let left_phrase = format!("{}_", luatnhan_phrase);
            let right_phrase = format!("_{}", luatnhan_phrase);
            if self.luatnhan_pair_phrases.contains_key(&left_phrase) {
                luatnhan_left_edges.insert(start, mat.clone());
                luatnhan_left_edges_value
                    .insert(start, self.luatnhan_pair_phrases.get(&left_phrase).unwrap());
            }
            if self.luatnhan_pair_phrases.contains_key(&right_phrase) {
                luatnhan_right_edges.insert(start, mat.clone());
                luatnhan_right_edges_value
                    .insert(start, self.luatnhan_pair_phrases.get(&right_phrase).unwrap());
            }
            if pre_mat.is_some() {
                let pre_match = pre_mat.unwrap();
                let phrase_key = format!(
                    "{}_{}",
                    &content[pre_match.start()..pre_match.end()],
                    luatnhan_phrase
                );
                if self.luatnhan_pair_phrases.contains_key(&phrase_key) {
                    let phrase = &content[pre_match.end()..start];
                    let mut extracted = "";
                    if self.names_map.contains_key(phrase) {
                        extracted = self.names_map.get(phrase).unwrap();
                    } else if self.pronouns_map.contains_key(phrase) {
                        extracted = self.pronouns_map.get(phrase).unwrap();
                    } else if self.vietphrase_map.contains_key(phrase) && phrase.len() <= 5 {
                        extracted = self.vietphrase_map.get(phrase).unwrap();
                    }
                    if !extracted.is_empty() {
                        let phrase_value = self.luatnhan_pair_phrases.get(&phrase_key).unwrap();
                        let translated = phrase_value.replace("{0}", extracted);
                        luatnhan_pairs.insert(pre_match.start(), translated);
                        luatnhan_idx_pairs.insert(pre_match.start(), end);
                    }
                }
            }
            pre_mat = Some(mat);
        }

        // build vietphrase aho corasick
        let ac = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(self.vietphrase_map.keys());

        let mut replacements = HashMap::new();
        let mut replacement_bit_vec = BitVec::from_elem(content.len(), false);
        for mat in ac.find_iter(content) {
            replacement_bit_vec.set(mat.start(), true);
            replacements.insert(mat.start(), mat);
        }

        let mut res = String::new();
        let mut last = 0;
        let mut previous_char = "";
        for char_index in content.grapheme_indices(true) {
            let i = char_index.0;
            if i < last {
                continue;
            }
            last = 0;
            let current_char = char_index.1;
            // adding space?
            if !previous_char.trim().is_empty() && previous_char != "\"" {
                let pc = previous_char.chars().next().unwrap();
                let cc = current_char.chars().next().unwrap();
                if (cc.is_alphabetic()) || (pc.is_alphabetic() && cc.is_alphanumeric()) {
                    res.push_str(" ");
                }
            }

            let mut found = false;

            if luatnhan_idx_pairs.contains_key(&i) {
                res.push_str(&*luatnhan_pairs.get(&i).unwrap().trim());
                last = *luatnhan_idx_pairs.get(&i).unwrap();
                found = true;
            }

            if !found && luatnhan_left_edges.contains_key(&i) {
                let next = luatnhan_left_edges.get(&i).unwrap().end();
                if replacement_bit_vec.get(next).unwrap() {
                    let mat = replacements.get(&next).unwrap();
                    let mat_str = &content[mat.start()..mat.end()];
                    let replace_str = self.vietphrase_map.get(mat_str).unwrap().trim();
                    let phrase_value = *luatnhan_left_edges_value.get(&i).unwrap();
                    let translated = phrase_value.replace("{0}", replace_str);
                    res.push_str(translated.trim());
                    found = true;
                    last = mat.end();
                }
            }

            if !found && replacement_bit_vec.get(i).unwrap() {
                let mat = replacements.get(&i).unwrap();
                let mat_str = &content[mat.start()..mat.end()];
                let replace_str = self.vietphrase_map.get(mat_str).unwrap().trim();
                let next = mat.end();
                if luatnhan_right_edges.contains_key(&next) {
                    let phrase_value = *luatnhan_right_edges_value.get(&next).unwrap();
                    let translated = phrase_value.replace("{0}", replace_str);
                    res.push_str(translated.trim());
                    last = luatnhan_right_edges.get(&next).unwrap().end();
                } else {
                    res.push_str(replace_str);
                    last = mat.end();
                }
                found = true;
            }

            if !found {
                let replace_str = self.hanviet_map.get(current_char);
                if replace_str.is_some() {
                    res.push_str(replace_str.unwrap());
                } else {
                    res.push_str(current_char);
                }
            }
            previous_char = current_char;
        }

        // normalize content
        let mut formalized_str = String::new();
        let mut begin_sentence = true;
        for char_index in res.grapheme_indices(true) {
            let char = char_index.1;
            if char == "\n" || char == "?" || char == "!" || char == "." {
                begin_sentence = true;
                formalized_str.push_str(char);
            } else if begin_sentence && char.chars().next().unwrap().is_alphabetic() {
                formalized_str.push_str(&char.to_uppercase());
                begin_sentence = false;
            } else {
                formalized_str.push_str(char);
            }
        }

        return formalized_str;
    }
}

fn load_dict(dict: &String, map: &mut HashMap<String, String>) {
    for line in dict.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut pairs = line.trim().split("=");
        let k = pairs.next().unwrap().trim();
        let v = pairs.next().unwrap().trim();
        let mut options = v.splitn(2, "/");
        let option1 = options.next().unwrap().trim();
        map.insert(k.to_string(), option1.to_string());
    }
}

#[wasm_bindgen]
impl Converter {
    pub fn set_vietphrase_dict(&mut self, vietphrase: String) {
        load_dict(&vietphrase, &mut self.vietphrase_map);
    }
    pub fn set_names_dict(&mut self, names: String) {
        load_dict(&names, &mut self.names_map);
    }
    pub fn set_hanviet_dict(&mut self, hanviet: String) {
        load_dict(&hanviet, &mut self.hanviet_map);
    }
    pub fn set_luatnhan_dict(&mut self, luatnhan: String) {
        load_dict(&luatnhan, &mut self.luatnhan_map);
    }
    pub fn set_pronouns_dict(&mut self, pronouns: String) {
        load_dict(&pronouns, &mut self.pronouns_map);
    }
}
