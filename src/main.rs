extern crate ansi_term;
extern crate clap;
extern crate huedump;

fn main() {
    #[cfg(windows)]
    let can_colour = ansi_term::enable_ansi_support();
    #[cfg(not(windows))]
    let can_colour = Result::<(), u32>::Ok(());

    let colour = {
        if let Ok(_) = can_colour {
            huedump::colours::colour
        } else {
            huedump::colours::leave_alone
        }
    };

    if let Err(err) = huedump::exec(colour) {
        eprintln!("{}\t{}", colour(0, String::from("Error!")), err);

        println!(
            "{}\tAre you sure that the file you specified is accessible?",
            colour(20, String::from("Info"))
        );
    }
}
