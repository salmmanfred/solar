use crate::html;
use std::path::Path;
use web_view::*;
// a simple window for errors
pub fn error(){
        web_view::builder()
            .title("Error")
            .content(Content::Html(htmlloader()))
            .size(100, 100)
            .resizable(true)
            .debug(false)
            .user_data(())
            .invoke_handler(|webview, arg| {
                match arg {
                    
                    "exit" => {
                        webview.exit();
                    }
                    _ => {
                        unimplemented!();
                    },
                };
                Ok(())
            })
            .run()
            .unwrap();
    
}
fn htmlloader() -> String{
    if Path::new("./src/error.html").exists(){
        println!("From html file");
        return openfile::readFile("src/error.html");

    }else{
        println!("From compiled");
        return html::erh.to_string();
    }

}