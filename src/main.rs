
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ffmpeg_controller")]
#[command(about = "assist ffmpeg cli app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "test process", aliases = ["t"])]
    TestProcess,

    #[command(about = "another test process", aliases = ["at"])]
    AnotherTestProcess,
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
      Some(Commands::TestProcess)=>{
          println!("test process");
      },

      Some(Commands::AnotherTestProcess)=>{
        println!("another test process");
    }
      None => {
          // 引数がない場合はヘルプを表示
          Cli::command().print_help()?;
          std::process::exit(0);
      }
  }

  Ok(())
}
