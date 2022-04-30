use aho_corasick::*;
use std::collections::HashMap;
use bit_vec::BitVec;
use unicode_segmentation::UnicodeSegmentation;

pub fn convert(vietphrase: &str, hanviet: &str, names: &str, content: &str) -> String {
    
    let mut vietphrase_map = HashMap::new();
    let mut hanviet_map = HashMap::new();

    vietphrase_map = load_dict(&vietphrase, vietphrase_map);
    vietphrase_map = load_dict(&names, vietphrase_map);
    hanviet_map = load_dict(&hanviet, hanviet_map);

    let ac = AhoCorasickBuilder::new().match_kind(MatchKind::LeftmostLongest).build(vietphrase_map.keys());
    
    let mut replacements = HashMap::new();
    let mut replacement_bit_vec = BitVec::from_elem(content.len(), false);
    for mat in ac.find_iter(content) {
      // let mat_str = &content[mat.start()..mat.end()];
      // let replace_str = vietphrase_map.get(mat_str).unwrap();
      replacement_bit_vec.set(mat.start(), true);
      replacements.insert(mat.start(), mat);
      // replacements.insert(mat.start(), *replace_str);
      // res = res.replace(mat_str, replace_str);
    }

    let mut res = String::new();
    let mut i = 0;
    let mut last = 0;
    for char_index in content.grapheme_indices(true) {
      let i = char_index.0;
      if i < last {
        continue;
      }
      last = 0;
      if replacement_bit_vec.get(i).unwrap() {
        let mat = replacements.get(&i).unwrap();
        let mat_str = &content[mat.start()..mat.end()];
        let replace_str = vietphrase_map.get(mat_str).unwrap();
        // let replace_str = *replacements.get(&i).unwrap();
        res.push_str(*replace_str);
        last = mat.end();
      } else {
        let single_char = char_index.1;
        let replace_str = hanviet_map.get(single_char);
        if replace_str.is_some() {
          res.push_str(*replace_str.unwrap());
        } else {
          res.push_str(single_char);
        }
      }
      res.push_str(" ");
    }

    // while i < content.len() {
    //   if replacement_bit_vec.get(i).unwrap() {
    //     let mat = replacements.get(&i).unwrap();
    //     let mat_str = &content[mat.start()..mat.end()];
    //     let replace_str = vietphrase_map.get(mat_str).unwrap();
    //     // let replace_str = *replacements.get(&i).unwrap();
    //     res.push_str(*replace_str);
    //     i = mat.end() + 1;
    //   } else {
    //     let single = &content[i..i+1];
    //     let replace_str = hanviet_map.get(single);
    //     if replace_str.is_some() {
    //       res.push_str(*replace_str.unwrap());
    //     } else {
    //       res.push_str(single);
    //     }
    //     i = i + 1;
    //   }
    //   res.push_str(" ");
      
    // }
    // for mat in ac.find_iter(content) {
    //   let mat_str = &content[mat.start()..mat.end()];
    //   let replace_str = vietphrase_map.get(mat_str);
    //   res = res.replace(mat_str, replace_str.unwrap());
    // }

    return res;
    
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