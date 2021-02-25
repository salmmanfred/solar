use crate::html;
use std::path::Path;
use web_view::*;
// a simple window for errors

pub fn error(){
    let html: html::html = html::html{
        html: "./src/html/error.html".to_string(),
        im: html::erh.to_string(),
    };
        web_view::builder()
            .title("Error")
            .content(Content::Html(html.html()))
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
