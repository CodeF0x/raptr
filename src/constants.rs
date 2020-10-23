    pub const HTML_BOILERPLATE: &str = "
    <!doctype html>
    <html>
    <head>
        <meta charset=\"utf-8\"/>
        <title>{{ document_name }}</title>
        <style>{{ document_style }}</style>
    </head>
    <body>
        {{ document_body }}
    <script>{{ document_js }}</script>
    </body>
    </html>";

pub const CSS_BOILERPLATE: &str = "";
