use super::options::GenOptions;

pub fn gen_svg(options: &GenOptions) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1500 500">
            <defs>
                <style>
                    @import url(https://fonts.bunny.net/css?family=mulish:700);

                    svg {{
                        background: {};
                    }}

                    .title {{
                        fill: #222;
                        font: 700 110px "Mulish", sans-serif;
                        
                        dominant-baseline: middle;
                        text-anchor: middle;

                        letter-spacing: 7px;
                    }}
                </style>
            </defs>
            <text class="title" x="50%" y="50%">
                {}
            </text>
        </svg>"#,
        options.color, options.title
    )
}
