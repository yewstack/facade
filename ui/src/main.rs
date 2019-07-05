use failure::Error;

fn main() -> Result<(), Error> {
    //web_logger::init();
    yew::start_app::<rillrate_ui::Model>();
    Ok(())
}
