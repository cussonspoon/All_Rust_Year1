use std::fs::File;
use std::io::Write;

struct XPM2Image {
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

fn make_xpm2(ctable: &[(String, String)], pixels: &[String]) -> XPM2Image {
    XPM2Image {
        colors: ctable.to_vec(),
        pixels: pixels.to_vec(),
    }
}

fn generate_svg(image: &XPM2Image, pixel_size: u32) -> String {
    let mut svg = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">
<style type="text/css">
rect {{ stroke: #00FFFF; }}
rect.c1 {{ fill: {color1}; }}
rect.c2 {{ fill: {color2}; }}
</style>
"#,
        width = image.pixels[0].len() as u32 * pixel_size,
        height = image.pixels.len() as u32 * pixel_size,
        color1 = image.colors[0].1, 
        color2 = image.colors[1].1   
    );

    for (y, row) in image.pixels.iter().enumerate() {
        for (x, pixel) in row.chars().enumerate() {
            match pixel {
                '#' => {
                    svg += &format!(
                        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" class="{class}"/>"#,
                        x = x as u32 * pixel_size,
                        y = y as u32 * pixel_size,
                        width = pixel_size,
                        height = pixel_size,
                        class = "c1"
                    );
                }
                '-' => {
                    svg += &format!(
                        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" class="{class}"/>"#,
                        x = x as u32 * pixel_size,
                        y = y as u32 * pixel_size,
                        width = pixel_size,
                        height = pixel_size,
                        class = "c2"
                    );
                }
                _ => {}
            }
        }
    }

    svg += "</svg>";
    svg
}

fn main() {
    let ctable = &[
        ("#".into(), "#000000".into()),
        ("-".into(), "#808080".into()),  
    ];
    let rows = ["##--", "-#"];
    let pixels: Vec<_> = rows.iter().map(|r| r.to_string()).collect();
    let img = make_xpm2(ctable, &pixels);

    let pixel_size = 20;

    let svg = generate_svg(&img, pixel_size);

    let mut file = File::create("output.svg").expect("Failed to create file");
    file.write_all(svg.as_bytes()).expect("Failed to write to file");
}
