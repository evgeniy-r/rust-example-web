use super::super::response::Body;

pub fn body(status: u16) -> Body {
    let title = crate::status_with_name(status);
    format!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                <meta charset=\"utf-8\" />
                <title>{}</title>
                </head>
                <body>
                    <h1>{}</h1>
                </body>
            </html>
        "#,
        title, title
    )
    .as_bytes()
    .to_vec()
}
