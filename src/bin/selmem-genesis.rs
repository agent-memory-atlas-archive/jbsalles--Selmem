//! Curated GENESIS qualification and cumulative-learning experiments.
#[path="selmem-genesis/http.rs"] mod http;
#[path="selmem-locomo/json.rs"] mod json;
#[path="selmem-locomo/sha256.rs"] mod sha256;
#[path="selmem-genesis/native.rs"] mod native;
#[path="selmem-genesis/progress.rs"] mod progress;
#[path="selmem-genesis/runner.rs"] mod runner;
#[path="selmem-genesis/transport.rs"] mod transport;
#[path="selmem-genesis/positive_control.rs"] mod positive_control;
#[path="selmem-genesis/learning.rs"] mod learning;
#[cfg(test)] #[path="selmem-genesis/lifecycle_tests.rs"] mod lifecycle_tests;
#[cfg(test)] #[path="selmem-genesis/tests.rs"] mod tests;
fn cli(mut args:Vec<String>)->Result<(),String>{
    if args.is_empty() || args.iter().any(|a|a=="--help"){
        println!("GENESIS: positive | learning\n./run.sh run --release --bin selmem-genesis -- positive --config .selmem --llm grok --model grok-4.3 --out DIR --max-http-calls 60\n./run.sh run --release --bin selmem-genesis -- learning --config .selmem --llm grok --model grok-4.3 --source QUALIFIED_DIR --out DIR --observation-evidence on|off --max-http-calls 800\nBoth commands support --mock. Mock runs are technical rehearsals. Outputs must be fresh.");return Ok(());
    }
    let command=args.remove(0);
    if !["positive","learning"].contains(&command.as_str()){return Err(format!("retired or unknown command {command}; use positive|learning --help"));}
    if let Some(i)=args.iter().position(|a|a=="--config"){
        let path=args.get(i+1).ok_or("--config requires a path")?;
        selmem::Config::load_path(std::path::Path::new(path)).map_err(|e|format!("runtime config: {e}"))?;
    }
    match command.as_str(){"positive"=>positive_control::cli(args),"learning"=>learning::cli(args),_=>unreachable!()}
}
fn main(){if let Err(e)=cli(std::env::args().skip(1).collect()){progress::line(format!("ERROR {e}"));eprintln!("GENESIS: {e}");std::process::exit(1);}}
