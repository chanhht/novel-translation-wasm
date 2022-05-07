mod converter;

use wasm_bindgen::prelude::*;
// use aho_corasick::AhoCorasick;
// use futures::executor;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// #[wasm_bindgen]
// extern "C" {
//     pub fn getMainDict() -> String;
// }

#[wasm_bindgen]
pub async fn convert(input: String) -> String {
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
    return converter::convert(
        &vietphrases.as_string().unwrap(),
        &names.as_string().unwrap(),
        &hanviet.as_string().unwrap(),
        &input,
    );
}

async fn load_vietphrase(url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // let url = getMainDict();

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/text")?;

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
    fn it_works_no_change() {
        assert_eq!("笨 成", convert("abc=edf", "abc=edf", "abc=edf", "笨成"));
    }

    #[test]
    fn it_works() {
        assert_eq!(
            "Bổn thành",
            convert("abc=edf", "abc=edf", "笨=bổn\n成=thành", "笨成")
        );
    }

    #[test]
    fn multiple_meaning() {
        assert_eq!(
            "Mau",
            convert("快=mau/khoái", "abc=edf", "", "快")
        );
    }
   
    #[test]
    fn load_big_file() {
        let vietphrase = fs::read_to_string("public/dicts/vietphrase.txt")
            .expect("Something went wrong reading the file");
        let names =
            fs::read_to_string("public/dicts/names.txt").expect("Something went wrong reading the file");
        let hanviet =
            fs::read_to_string("public/dicts/hanviet.txt").expect("Something went wrong reading the file");
        assert_eq!(
            "Thứ nhất chương thái dương biến mất()\nThời gian:2012 niên 12 nguyệt 22 nhật",
            convert(
                &vietphrase,
                &names,
                &hanviet,
                "第一章 太阳消失()\n时间:2012年12月22日"
            )
        );
        assert_eq!(
            "Của ta giá chương rất lớn， thỉnh nhẫn một chút！",
            convert(
                &vietphrase,
                &names,
                &hanviet,
                "我的这章很大，请忍一下！"
            )
        );
        assert_eq!(
            "Khoái nghĩ biện pháp a！ chúng ta sẽ không chết đâu",
            convert(
                &vietphrase,
                &names,
                &hanviet,
                "快想办法啊！我们会没命的"
            )
        );

    }
}
