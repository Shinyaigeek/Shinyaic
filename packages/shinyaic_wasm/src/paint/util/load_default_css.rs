pub fn load_default_css() -> String {
    return "h1 {{
        display: block;
        font-size: 2em;
        margin-top: 0.67em;
        margin-bottom: 0.67em;
        margin-left: 0;
        margin-right: 0;
        font-weight: bold;
      }}
      body {{
        min-height: 100vh;
      }}
      header {{
        display: block;
      }}
      nav {{
        display: block;
      }}"
    .to_string();
}
