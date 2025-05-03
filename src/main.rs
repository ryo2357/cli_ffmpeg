
use clap::{CommandFactory, Parser, Subcommand};

// Cargo.tomlからの定数のロード
// 一回ビルドしたらrust_analyzerのエラーは消える
include!(concat!(env!("OUT_DIR"), "/package_info.rs"));

mod config;



#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = APP_DESCRIPTION)]
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
    let config = config::Config::load();

    match &cli.command {
      Some(Commands::TestProcess)=>{
          println!("test process");
          println!("config: {:?}", config);
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
