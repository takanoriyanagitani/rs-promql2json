use std::io;
use std::process::ExitCode;

use io::BufWriter;
use io::Write;

use io::Read;

use rs_promql2json::lmtd_reader2parsed2json2writer;

fn envkey2qlsz_max_s(key: String) -> impl Fn() -> Result<String, io::Error> {
    move || {
        std::env::var(&key)
            .map_err(|e| format!("promql size max {key} unknown: {e}"))
            .map_err(io::Error::other)
    }
}

fn qlsz_s2u(qlsz: &str) -> Result<u64, io::Error> {
    str::parse(qlsz).map_err(io::Error::other)
}

fn envkey2qlsz_max(key: String) -> impl Fn() -> Result<u64, io::Error> {
    let env2qlsz_s = envkey2qlsz_max_s(key);
    move || {
        let qlsz_s: String = env2qlsz_s()?;
        qlsz_s2u(&qlsz_s)
    }
}

fn sub() -> Result<(), io::Error> {
    let qlsz: u64 = envkey2qlsz_max("ENV_PROMQL_SIZE_MAX".into())()?;
    let lmtd = std::io::stdin().lock().take(qlsz);

    let o = std::io::stdout();
    let mut ol = o.lock();
    let mut bw = BufWriter::new(&mut ol);

    let mut parsed2wtr = lmtd_reader2parsed2json2writer(lmtd);
    parsed2wtr(&mut bw)?;
    drop(parsed2wtr);
    bw.flush()?;
    drop(bw);

    ol.flush()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
