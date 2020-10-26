pub const HTML_BOILERPLATE: &str = "
    <!doctype html>
    <html>
    <head>
        <meta charset=\"utf-8\"/>
        <title>{{ document_name }}</title>
        <style>{{ document_style }}</style>
    </head>
    <body>
        <section class=\"middle\">
            {{ document_body }}
        </section>
    </body>
    </html>";

pub const CSS_BOILERPLATE: &str = "
    body {
        background: #292a2d;
        color: #a9a9b3;
        font-family: 'Inter UI', -apple-system, BlinkMacSystemFont, \"Roboto\", \"Segoe UI\", Helvetica, Arial, sans-serif
    }

    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
        text-decoration: underline;
    }

    a {
        color: unset;
    }

    .middle {
        width: 70%;
        margin: 0 auto;
    }

    blockquote {
        background: #686868;
        border-left: 10px solid #ccc;
        margin: 1.5em 10px;
        padding: 0.5em 10px;
        quotes: \"\\201C\"\"\\201D\"\"\\2018\"\"\\2019\";
      }

      blockquote::before {
        color: #ccc;
        content: open-quote;
        font-size: 4em;
        line-height: 0.1em;
        margin-right: 0.25em;
        vertical-align: -0.4em;
      }

      blockquote p {
        display: inline;
        color: white;
      }

      pre {
          overflow-x: auto;
          background: #403f3f;
          color: lightgrey;
      }

    @media (prefers-color-scheme: light) {
        body {
            background: white;
            color: black;
        }

        pre {
            background: #cac8c8;
            color: black;
        }

        blockquote {
            background: #cac8c8;
            border-left: 10px solid black;
        }

        blockquote p,
        blockquote::before {
            color: black;
        }
    }
";
