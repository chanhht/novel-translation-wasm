use aho_corasick::AhoCorasick;
use std::collections::HashMap;

pub fn convert(dicts: &str, content: &str) -> String {
    let lines = dicts.split("\n");
    let mut map = HashMap::new();
    
    // let patterns = &["Samwise", "Sam"];

    for line in lines {
      if line.is_empty() {
        continue;
      }
      let mut pairs = line.split("=");
      let k = pairs.next().unwrap();
      let v = pairs.next().unwrap();
      // println!("{} = {}", k , v);
      map.insert(k, v);
    }

    let ac = AhoCorasick::new(map.keys());

    let mut res = String::from(content);
    for mat in ac.find_iter(content) {
      let mat_str = &content[mat.start()..mat.end()];
      let replace_str = map.get(mat_str);
      res = res.replace(mat_str, replace_str.unwrap());
    }
    
    return res;
    
}