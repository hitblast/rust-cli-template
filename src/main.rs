use clap::Parser;
use template::{
    cli::{
        Args,
        args::{Command, HelloSubcmd},
        atomic::{set_accept_all, set_dry_run, set_quiet, set_verbose},
    },
    commands::Runnable,
    log_err,
    utils::sudo::run_with_noroot,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = Args::parse();

    // sudo protection
    let result = match &args.command {
        // Command::SelfUpdate(_) => run_with_root(),
        _ => run_with_noroot(),
    };

    // set global flags
    set_accept_all(args.accept_all);
    set_quiet(args.quiet);
    set_verbose(args.verbose);
    set_dry_run(args.dry_run);

    if let Err(err) = result {
        log_err!("Invoke failure: {err}");
        std::process::exit(1);
    }

    // command invocation
    let result = match &args.command {
        Command::Greet(cmd) => cmd.run().await,
        Command::Hello { command } => match command {
            HelloSubcmd::World(cmd) => cmd.run().await,
        },
    };

    if let Err(err) = result {
        log_err!("{err}");
        std::process::exit(1);
    }
}
