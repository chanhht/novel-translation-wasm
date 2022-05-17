mod converter;

#[cfg(test)]
mod tests {

    use std::fs;

    use super::converter::Converter;

    #[test]
    fn it_works_no_change() {
        let mut converter = Converter::new();
        assert_eq!(
            "笨 成",
            converter.convert(&"笨成".to_owned())
        );
    }

    #[test]
    fn it_works() {
        let mut converter = Converter::new();
        converter.set_vietphrase_dict("笨=bổn\n成=thành".to_owned());
        assert_eq!(
            "Bổn thành",
            converter.convert(&"笨成".to_owned())
        );
    }

    #[test]
    fn multiple_meaning() {
        let mut converter = Converter::new();
        converter.set_hanviet_dict("快=mau/khoái".to_owned());
        assert_eq!("Mau", converter.convert(&"快".to_owned()));
    }

    #[test]
    fn load_big_file() {
        let mut converter = Converter::new();
        // converter.load_dictionary();
        let vietphrase = fs::read_to_string("public/dicts/vietphrase.txt")
            .expect("Something went wrong reading the file");
        let names = fs::read_to_string("public/dicts/names.txt")
            .expect("Something went wrong reading the file");
        let hanviet = fs::read_to_string("public/dicts/hanviet.txt")
            .expect("Something went wrong reading the file");
        let luatnhan = fs::read_to_string("public/dicts/luatnhan.txt")
            .expect("Something went wrong reading the file");
        let pronouns = fs::read_to_string("public/dicts/pronouns.txt")
            .expect("Something went wrong reading the file");
        converter.set_hanviet_dict(hanviet);
        converter.set_luatnhan_dict(luatnhan);
        converter.set_names_dict(names);
        converter.set_pronouns_dict(pronouns);
        converter.set_vietphrase_dict(vietphrase);
        assert_eq!(
            "Chương một Thái Dương biến mất\nThời gian:2012 năm 12 nguyệt 22 ngày",
            converter.convert(&"第一章 太阳消失\n时间:2012年12月22日".to_owned())
        );
        assert_eq!(
            "Quân đoàn Nỗ Mễ Khải Lạp Khắc",
            converter.convert(&"努米凯拉克军团".to_owned())
        );
        assert_eq!(
            "Làm ta bị thương nặng",
            converter.convert(&"重伤了我".to_owned())
        );
        assert_eq!(
            "Này của ta chương rất lớn， thỉnh nhẫn một chút！",
            converter.convert(&"我的这章很大，请忍一下！".to_owned())
        );
        assert_eq!(
            "Mau nghĩ biện pháp a！ chúng ta sẽ không chết đâu",
            converter.convert(&"快想办法啊！我们会没命的".to_owned())
        );
    }
}
