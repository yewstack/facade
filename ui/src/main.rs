use failure::Error;

fn main() -> Result<(), Error> {
    //web_logger::init();
    yew::start_app::<facade_ui::Model>();
    Ok(())
}
