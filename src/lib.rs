use std::io;

use io::Read;
use io::Write;

use promql_parser::parser::ast::Expr;

pub fn promql2parsed(ql: &str) -> Result<Expr, String> {
    promql_parser::parser::parse(ql)
}

pub fn parsed2json2writer<W>(mut wtr: W) -> impl FnMut(&Expr) -> Result<(), io::Error>
where
    W: Write,
{
    move |parsed: &Expr| serde_json::to_writer(&mut wtr, parsed).map_err(io::Error::other)
}

pub fn lmtd_reader2string<R>(mut rdr: R) -> impl FnMut() -> Result<String, io::Error>
where
    R: Read,
{
    move || {
        let mut buf: String = String::default();
        rdr.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

pub fn lmtd_reader2parsed<R>(rdr: R) -> impl FnMut() -> Result<Expr, io::Error>
where
    R: Read,
{
    let mut rdr2str = lmtd_reader2string(rdr);
    move || {
        let promql: String = rdr2str()?;
        promql2parsed(&promql).map_err(io::Error::other)
    }
}

pub fn lmtd_reader2parsed2json2writer<R, W>(rdr: R) -> impl FnMut(W) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let mut rdr2parsed = lmtd_reader2parsed(rdr);
    move |wtr: W| {
        let mut parsed2json2wtr = parsed2json2writer(wtr);
        let parsed: Expr = rdr2parsed()?;
        parsed2json2wtr(&parsed)
    }
}
