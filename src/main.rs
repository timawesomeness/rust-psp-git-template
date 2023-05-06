#![no_std]
#![no_main]

psp::module!("{{project-name}}", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    psp::dprint!("Hello PSP from rust!");
}
