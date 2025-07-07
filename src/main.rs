pub mod uevent;
pub mod ui;

fn main() -> iced::Result {
    // let battery = uevent::Uevent::new(r"C:\Users\Миша\uevent").unwrap();
    // println!("{battery:#?}");
    ui::ui()
}
