mod converter;

use wasm_bindgen::prelude::*;
// use aho_corasick::AhoCorasick;
// use futures::executor;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    pub fn getContent() -> String;
    pub fn getMainDict() -> String;
    // pub async fn loadVietphrase() -> JsValue;
}


#[wasm_bindgen]
pub async fn convert() -> String {
    let vietphrases_future = load_vietphrase("dicts/vietphrase.txt");
    let names_future = load_vietphrase("dicts/names.txt");
    let hanviet_future = load_vietphrase("dicts/hanviet.txt");
    
    let vietphrases = vietphrases_future.await.expect("something wrong");
    let names = names_future.await.expect("something wrong");
    let hanviet = hanviet_future.await.expect("something wrong");

    // let names = load_vietphrase("dicts/names.txt").await;
    // let hanviet = load_vietphrase("dicts/hanviet.txt").await;
    // res = converter::convert(&vietphrases.as_string().unwrap(), &content);
    // let vietphrases = executor::block_on(loadVietphrase().await?);
    return converter::convert(&vietphrases.as_string().unwrap(), 
                    &names.as_string().unwrap(), 
                    &hanviet.as_string().unwrap(), &getContent())
}

async fn load_vietphrase(url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // let url = getMainDict();

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/text")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?;

    Ok(text)

}

#[cfg(test)]
mod tests {

    use super::converter::*;
    use std::fs;

    #[test]
    fn it_works() {
        assert_eq!("edf fgh", convert("abc=edf", "abc=edf", "abc=edf", "abc fgh"));
    }

    #[test]
    fn it_works_no_change() {
        assert_eq!("hello get content", convert("abc=edf", "abc=edf", "abc=edf", "hello get content"));
    }

    #[test]
    fn load_big_file() {
        let vietphrase = fs::read_to_string("dicts/vietphrase.txt")
            .expect("Something went wrong reading the file");
        let names = fs::read_to_string("dicts/names.txt")
            .expect("Something went wrong reading the file");
        let hanviet = fs::read_to_string("dicts/hanviet.txt")
            .expect("Something went wrong reading the file");
        assert_eq!("hello get content", convert(&vietphrase, &names, &hanviet, "hello get content"));
    }
}