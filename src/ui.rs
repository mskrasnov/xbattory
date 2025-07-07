//! Graphic user interface based on `iced`

use iced::{
    Alignment::Center,
    Color, Element, Settings, Theme,
    alignment::Horizontal,
    widget::{button, center, column, container, horizontal_space, image, row, svg, text},
};

use crate::uevent::{Status, Uevent};

pub fn ui() -> iced::Result {
    iced::application("XBattory", XBattory::update, XBattory::view)
        .settings(Settings {
            default_text_size: iced::Pixels(12.),
            ..Default::default()
        })
        .window_size([430., 290.])
        .antialiasing(true)
        .theme(XBattory::theme)
        .run()
}

struct XBattory {
    current_page: Page,
    previous_page: Option<Page>,
    uevent: Option<Uevent>,
}

#[derive(Debug, Clone)]
enum Message {
    SelectPage(Page),
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
enum Page {
    #[default]
    Main,
    Details,
    Statistics,
    Settings,
    About,
}

impl Page {
    const SVG_BATTERY_STATUS_SIZE: u16 = 64;

    fn get_bat_status_icon(&self, p: u8) -> &str {
        match p {
            0..=5 => "0",
            6..=10 => "10",
            11..=20 => "20",
            21..=30 => "30",
            31..=40 => "40",
            41..=50 => "50",
            51..=60 => "60",
            61..=70 => "70",
            71..=80 => "80",
            81..=90 => "90",
            _ => "100",
        }
    }

    fn main<'a>(&'a self, uevent: &'a Option<Uevent>) -> Element<'a, Message> {
        let header = if let Some(uevent) = uevent {
            let health = row![
                text(format!("Health: {:.2}%", uevent.health)),
                match uevent.health as u8 {
                    0..=50 => row![
                        image("./icons/emblem-dead.png"),
                        text("Battery is dead :-(")
                    ]
                    .spacing(5),
                    51..=80 => row![
                        image("./icons/emblem-danger.png"),
                        text("Battery looks like unhealthy!")
                    ]
                    .spacing(5),
                    _ => row![image("./icons/emblem-ok.png"), text("Battery is OK!")].spacing(5),
                }
            ]
            .spacing(5);

            row![
                svg(format!(
                    "./icons/status/battery-level-{}{}-symbolic.svg",
                    self.get_bat_status_icon(uevent.capacity),
                    match uevent.status {
                        Status::Charging => "-charging",
                        _ => "",
                    }
                ))
                .height(Self::SVG_BATTERY_STATUS_SIZE)
                .width(Self::SVG_BATTERY_STATUS_SIZE),
                column![
                    text(&uevent.model_name).size(15),
                    column![
                        text(format!("{} | {}%", uevent.status, uevent.capacity)),
                        health,
                    ]
                    .spacing(2),
                ]
                .spacing(5),
            ]
            .spacing(10)
        } else {
            row![
                svg("./icons/status/battery-missing-symbolic.svg")
                    .height(Self::SVG_BATTERY_STATUS_SIZE)
                    .width(Self::SVG_BATTERY_STATUS_SIZE),
                column![
                    text("Unknown battery model").size(15),
                    text("Failed to get information about your battery!"),
                ]
                .spacing(5),
            ]
            .spacing(10)
        };

        container(center(header.align_y(Center)))
            // .style(container::rounded_box)
            .into()
    }

    fn details<'a>(&'a self, uevent: &'a Option<Uevent>) -> Element<'a, Message> {
        match uevent {
            Some(uevent) => {
                let headers = column![
                    text("Description").color(Color::WHITE.scale_alpha(0.5)),
                    text("Name").color(Color::WHITE.scale_alpha(0.5)),
                    text("Model").color(Color::WHITE.scale_alpha(0.5)),
                    text("Manufacturer").color(Color::WHITE.scale_alpha(0.5)),
                    text("Serial Number").color(Color::WHITE.scale_alpha(0.5)),
                    text("Technology").color(Color::WHITE.scale_alpha(0.5)),
                    text("Status").color(Color::WHITE.scale_alpha(0.5)),
                    text("Capacity").color(Color::WHITE.scale_alpha(0.5)),
                    text("Capacity level").color(Color::WHITE.scale_alpha(0.5)),
                    text("Voltage min (by design)").color(Color::WHITE.scale_alpha(0.5)),
                    text("Voltage (now)").color(Color::WHITE.scale_alpha(0.5)),
                    text("Power (now)").color(Color::WHITE.scale_alpha(0.5)),
                    text("Energy full").color(Color::WHITE.scale_alpha(0.5)),
                    text("Energy full (by design)").color(Color::WHITE.scale_alpha(0.5)),
                    text("Energy (now)").color(Color::WHITE.scale_alpha(0.5)),
                    text("Battery health").color(Color::WHITE.scale_alpha(0.5)),
                ]
                .align_x(Horizontal::Right);
                let values = column![
                    text("Value").color(Color::WHITE.scale_alpha(0.5)),
                    text(&uevent.name),
                    text(&uevent.model_name),
                    text(&uevent.manufacturer),
                    text(&uevent.serial_number),
                    text(&uevent.technology),
                    text(format!("{}", uevent.status)),
                    text(format!("{}%", uevent.capacity)),
                    text(format!("{}", uevent.capacity_level)),
                    text(format!(
                        "{:.3}",
                        uevent.voltage_min_design as f64 * 0.000001f64
                    )),
                    text(format!("{:.3}", uevent.voltage_now as f64 * 0.000001f64)),
                    text(uevent.power_now),
                    // energy: Вт*ч
                    text(format!("{:.3}", uevent.energy_full as f64 * 0.000001f64)),
                    text(format!(
                        "{:.3}",
                        uevent.energy_full_design as f64 * 0.000001f64
                    )),
                    text(format!("{:.3}", uevent.energy_now as f64 * 0.000001f64)),
                    text(format!("{:.2}%", uevent.health)),
                ]
                .align_x(Horizontal::Left);

                container(row![
                    horizontal_space(),
                    row![headers, values].spacing(5),
                    horizontal_space()
                ])
                .into()
            }
            None => container(center(
                column![
                    svg("./icons/unimplemented.svg").width(128).height(128),
                    text("Failed to get information about battery!"),
                ]
                .align_x(Center)
                .spacing(5),
            ))
            .into(),
        }
    }

    fn about<'a>(&'a self) -> Element<'a, Message> {
        let header = row![
            image("./icons/logo.png").width(64).height(64),
            column![
                text("XBattory").size(15),
                text(format!("Version: {}", env!("CARGO_PKG_VERSION"))),
            ]
            .spacing(5),
        ]
        .align_y(Center)
        .spacing(5);

        let about_text = column![
            text("XBattory is a simple program to get information about notebook's battery"),
            text("Author: Michail Krasnov <michail383krasnov@mail.ru>"),
            text("\nYou can send me a donation to bank card:"),
            text("    2202 2062 5233 5406 (Sber; Russia)"),
            text("Thank you!"),
        ]
        .spacing(5);

        container(column![header, about_text].spacing(5)).into()
    }

    fn unimplemented_yet<'a>(&'a self) -> Element<'a, Message> {
        container(center(
            column![
                svg("./icons/unimplemented.svg").width(128).height(128),
                text("This function is unimplemented yet!"),
            ]
            .align_x(Center)
            .spacing(5),
        ))
        .into()
    }

    pub fn view<'a>(&'a self, uevent: &'a Option<Uevent>) -> Element<'a, Message> {
        match self {
            Self::Main => self.main(uevent),
            Self::Details => self.details(uevent),
            Self::About => self.about(),
            _ => self.unimplemented_yet(),
        }
    }
}

impl Default for XBattory {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            previous_page: None,
            uevent: match Uevent::new(r"C:\Users\Миша\uevent") {
                Ok(uevent) => Some(uevent),
                Err(_) => None,
            },
        }
    }
}

impl XBattory {
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectPage(page) => {
                if page == Page::Settings || page == Page::About {
                    self.previous_page = Some(self.current_page.clone());
                }
                self.current_page = page;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let page_selector = row![
            button(row![image("./icons/main.png").width(16).height(16), text("Main"),].spacing(2))
                .style(button::text)
                .on_press(Message::SelectPage(Page::Main)),
            button(
                row![
                    image("./icons/details.png").width(16).height(16),
                    text("Details"),
                ]
                .spacing(2)
            )
            .style(button::text)
            .on_press(Message::SelectPage(Page::Details)),
            button(
                row![
                    image("./icons/statistics.png").width(16).height(16),
                    text("Statistics"),
                ]
                .spacing(2)
            )
            .style(button::text)
            .on_press(Message::SelectPage(Page::Statistics)),
            horizontal_space(),
            button(
                row![
                    image("./icons/settings.png").width(16).height(16),
                    text("Settings"),
                ]
                .spacing(2)
            )
            .style(button::text)
            .on_press(Message::SelectPage(Page::Settings)),
            button(
                row![
                    image("./icons/emblem-question.png").width(16).height(16),
                    text("About")
                ]
                .spacing(2)
            )
            .style(button::text)
            .on_press(Message::SelectPage(Page::About)),
        ]
        .spacing(5);

        container(
            column![
                container(page_selector).style(container::rounded_box),
                self.current_page.view(&self.uevent),
            ]
            .spacing(5),
        )
        .padding(5)
        .into()
    }
}
