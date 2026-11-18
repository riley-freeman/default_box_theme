use box_blocks::{BoxDisplayName, threads::BoxThreads};
use mercy::{alloc::AllocatesTypes, context::Context};
use turubai::{
    Application,
    Unit::Em,
    color::Color,
    composition::{HorizontalAlignment, Spacer, VStack},
    elements::{ChildrenFunction, Element, Image, Modifiers, Picture, PictureFit, Text},
    font::{Font, FontWeight},
    postprocessing::{background_color, frame, padding},
    runtime::{Renderer, WindowTemplate},
};
use turubai_macros::markups;

#[derive(Default)]
pub struct MyApplication;

impl Application for MyApplication {
    fn id(&self) -> &'static str {
        "org.example.hello_world"
    }

    #[markups]
    fn markup(&self, renderer: Renderer, mut memory_context: Context) -> mercy::boxed::Box<dyn Element> {
        let large_font = Font::new(&mut memory_context, "Inter", 32, FontWeight::Bold, false);
        let font = Font::new(&mut memory_context, "Inter", 16, FontWeight::Regular, false);
        let image = Image::from_path("/resource/Sticker.png", &mut memory_context);

        let white = Color::with_code(0xffffff);
        let highlight_color = Color::new(120, 93, 65);

        let subheader_font = Font::new(&mut memory_context, "Inter", 12, FontWeight::Bold, false);

        turubai_macros::markup! (renderer: renderer.clone(), memory_context: memory_context.clone(), {
            WindowTemplate(title: "Turubai - Hello, World! (Example)") {
                VStack(alignment: HorizontalAlignment::Center) {
                    BoxDisplayName(text::font: large_font),
                    Text("Creator of the Crayon Box.", font: font.clone(), color: white)
                        .background_color(highlight_color),
                    Picture(image, fit: PictureFit::Contain)
                        .frame(width: Em(10.0), height: Em(10.0)),

                    Text("Latest Thread", font: subheader_font)
                    BoxThreads(0, 1),
                }
                .padding(all: Em(1.0))
            }
        })
    }
}

#[unsafe(no_mangle)]
fn render_profile() {
    let application = MyApplication::default();
    turubai::runtime::turubai_main(application);
}
