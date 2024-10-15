use db2md::gui::Db2Md;
use iced::Theme;

fn main() -> iced::Result
{
  let app = Db2Md::new();
  iced::application(app.title(), Db2Md::update, Db2Md::view).theme(|_| {
                                                          Theme::Dark
                                                        })
                                                        .centered()
                                                        .run()
}
