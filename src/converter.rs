use aho_corasick::AhoCorasick;
use std::collections::HashMap;

pub fn convert(dicts: &str, content: &str) -> String {
    let lines = dicts.split("\n");
    let mut map = HashMap::new();
    
    // let patterns = &["Samwise", "Sam"];

    for line in lines {
      let mut pairs = line.split("=");
      let k = pairs.next().unwrap();
      let v = pairs.next().unwrap();
      map.insert(k, v);
      println!("{} = {}", k , v)
    }

    let ac = AhoCorasick::new(map.keys());
    let mat = ac.find(content).expect("should have a match");
    let mat_str = &content[mat.start()..mat.end()];
    let replace_str = map.get(mat_str);
    return content.replace(mat_str, replace_str.unwrap());
    
}