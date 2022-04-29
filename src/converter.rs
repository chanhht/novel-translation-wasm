use aho_corasick::*;
use std::collections::HashMap;

pub fn convert(vietphrase: &str, hanviet: &str, names: &str, content: &str) -> String {
    
    let mut vietphrase_map = HashMap::new();
    let mut hanviet_map = HashMap::new();

    // for line in vietphrase.split("\n") {
    //   if line.is_empty() {
    //     continue;
    //   }
    //   let mut pairs = line.split("=");
    //   let k = pairs.next().unwrap();
    //   let v = pairs.next().unwrap();
    //   // println!("{} = {}", k , v);
    //   vietphraseMap.insert(k, v);
    // }

    vietphrase_map = load_dict(&vietphrase, vietphrase_map);
    vietphrase_map = load_dict(&names, vietphrase_map);
    hanviet_map = load_dict(&hanviet, hanviet_map);

    // for line in names.split("\n") {
    //   if line.is_empty() {
    //     continue;
    //   }
    //   let mut pairs = line.split("=");
    //   let k = pairs.next().unwrap();
    //   let v = pairs.next().unwrap();
    //   // println!("{} = {}", k , v);
    //   vietphraseMap.insert(k, v);
    // }

    // for line in hanviet.split("\n") {
    //   if line.is_empty() {
    //     continue;
    //   }
    //   let mut pairs = line.split("=");
    //   let k = pairs.next().unwrap();
    //   let v = pairs.next().unwrap();
    //   // println!("{} = {}", k , v);
    //   hanvietMap.insert(k, v);
    // }

    let ac = AhoCorasickBuilder::new().match_kind(MatchKind::LeftmostLongest).build(vietphrase_map.keys());

    let mut res = String::from(content);
    for mat in ac.find_iter(content) {
      let mat_str = &content[mat.start()..mat.end()];
      let replace_str = vietphrase_map.get(mat_str);
      res = res.replace(mat_str, replace_str.unwrap());
    }
    
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
    // println!("{} = {}", k , v);
    map.insert(k, v);
  }
  return map;
}