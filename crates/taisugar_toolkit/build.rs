const LANG_CHINESE_TRADITIONAL: u16 = 0x7c04;
const SUBLANG_CHINESE_TRADITIONAL: u16 = 0x01;

fn make_lang_id(lang: u16, sublang: u16) -> u16 {
    (sublang << 10) | lang
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set("ProductName", "台糖工具包");
        res.set("CompanyName", "趙子賢");
        res.set("LegalCopyright", "Copyright © 2025");
        res.set_icon("resources/windows/app-icon.ico");
        res.set_language(make_lang_id(
            LANG_CHINESE_TRADITIONAL,
            SUBLANG_CHINESE_TRADITIONAL,
        ));
        res.compile().unwrap();
    }
}
