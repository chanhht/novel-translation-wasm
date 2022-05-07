use aho_corasick::*;
use bit_vec::BitVec;
use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

pub fn convert(luatnhan: &str, vietphrase: &str, hanviet: &str, names: &str, pronouns: &str, content: &str) -> String {
    let mut luatnhan_map = HashMap::new();
    let mut vietphrase_map = HashMap::new();
    let mut hanviet_map = HashMap::new();
    let mut pronouns_map = HashMap::new();

    luatnhan_map = load_dict(&luatnhan, luatnhan_map);
    vietphrase_map = load_dict(&vietphrase, vietphrase_map);
    vietphrase_map = load_dict(&names, vietphrase_map);
    hanviet_map = load_dict(&hanviet, hanviet_map);
    pronouns_map = load_dict(&pronouns, pronouns_map);

    // build luatnhan aho corasick
    let mut luatnhan_phrases = HashSet::new();
    let mut luatnhan_pair_phrases = HashMap::new();
    for (k, v)  in luatnhan_map.iter() {
        let mut ps = k.splitn(2, "{0}");
        let ps1 = ps.next().unwrap();
        let ps2 = ps.next().unwrap();
        if !ps1.is_empty() {
            luatnhan_phrases.insert(ps1);
        }
        if !ps2.is_empty() {
            luatnhan_phrases.insert(ps2);
        }
        luatnhan_pair_phrases.insert(format!("{}_{}", ps1, ps2), *v);
    }

    let aho_corasick_luatnhan = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build(luatnhan_phrases);

    let mut pre_mat: Option<Match> = None;
    let mut luatnhan_pairs = HashMap::new();
    let mut luatnhan_right_edges = HashMap::new();
    let mut luatnhan_left_edges = HashMap::new();
    let mut luatnhan_right_edges_value = HashMap::new();
    let mut luatnhan_left_edges_value = HashMap::new();
    let mut luatnhan_idx_pairs = HashMap::new();
    let mut vec = Vec::new();
    for mat in aho_corasick_luatnhan.find_iter(content) {
        vec.push(mat);
    }
    vec.sort_by(|a, b| a.start().cmp(&b.start()));
    for mat in vec {
        let luatnhan_phrase = &content[mat.start()..mat.end()];
        let start = mat.start();
        let end = mat.end();
        let left_phrase = format!("{}_", luatnhan_phrase);
        let right_phrase = format!("_{}", luatnhan_phrase);
        if luatnhan_pair_phrases.contains_key(&left_phrase) {
            luatnhan_left_edges.insert(start, mat.clone());
            luatnhan_left_edges_value.insert(start, *luatnhan_pair_phrases.get(&left_phrase).unwrap());
        }
        if luatnhan_pair_phrases.contains_key(&right_phrase) {
            luatnhan_right_edges.insert(start, mat.clone());
            luatnhan_right_edges_value.insert(start, *luatnhan_pair_phrases.get(&right_phrase).unwrap());
        }
        if pre_mat.is_some() {
            let pre_match = pre_mat.unwrap();
            let phrase_key = format!("{}_{}", &content[pre_match.start()..pre_match.end()], luatnhan_phrase);
            if luatnhan_pair_phrases.contains_key(&phrase_key) {
                let phrase = &content[pre_match.end()..start];
                let mut extracted = "";
                if vietphrase_map.contains_key(phrase) {
                    extracted = vietphrase_map.get(phrase).unwrap();
                } else if pronouns_map.contains_key(phrase) {
                    extracted = pronouns_map.get(phrase).unwrap();
                }
                if !extracted.is_empty() {
                    let phrase_value = *luatnhan_pair_phrases.get(&phrase_key).unwrap();
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
        .build(vietphrase_map.keys());

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
            res.push_str(&*luatnhan_pairs.get(&i).unwrap().trim_end());
            last = *luatnhan_idx_pairs.get(&i).unwrap();
            found = true;
        } 
        
        if !found && luatnhan_left_edges.contains_key(&i) {
            let next = luatnhan_left_edges.get(&i).unwrap().end();
            if replacement_bit_vec.get(next).unwrap() {
                let mat = replacements.get(&next).unwrap();
                let mat_str = &content[mat.start()..mat.end()];
                let replace_str = vietphrase_map.get(mat_str).unwrap();
                let phrase_value = *luatnhan_left_edges_value.get(&i).unwrap();
                let translated = phrase_value.replace("{0}", *replace_str);
                res.push_str(translated.trim_end());
                found = true;
                last = mat.end();
            }
        } 
        
        if !found && replacement_bit_vec.get(i).unwrap() {
            let mat = replacements.get(&i).unwrap();
            let mat_str = &content[mat.start()..mat.end()];
            let replace_str = vietphrase_map.get(mat_str).unwrap();
            let next = mat.end();
            if luatnhan_right_edges.contains_key(&next) {
                let phrase_value = *luatnhan_right_edges_value.get(&next).unwrap();
                let translated = phrase_value.replace("{0}", *replace_str);
                res.push_str(translated.trim_end());
                last = luatnhan_right_edges.get(&next).unwrap().end();
            } else {
                res.push_str(&*replace_str.trim_end());
                last = mat.end();
            }
            found = true;
        }
        
        if !found {
            let replace_str = hanviet_map.get(current_char);
            if replace_str.is_some() {
                res.push_str(*replace_str.unwrap());
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
        } else if begin_sentence && char.chars().next().unwrap().is_lowercase() {
            formalized_str.push_str(&char.to_uppercase());
            begin_sentence = false;
        } else {
            formalized_str.push_str(char);
        }
    }

    return formalized_str;
}

fn load_dict<'a>(dict: &'a str, mut map: HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
    for line in dict.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut pairs = line.split("=");
        let k = pairs.next().unwrap();
        let v = pairs.next().unwrap();
        let mut options = v.splitn(2, "/");
        let option1 = options.next().unwrap();
        // println!("{} = {}", k , v);
        map.insert(k, option1);
    }
    return map;
}
