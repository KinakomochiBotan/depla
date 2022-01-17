use anyhow::{
    Result,
    bail,
    ensure
};

use pyo3::Python;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();

    let number: u32 = match args.next() {
        Option::Some(number) => number.parse()?,
        Option::None => bail!("please enter an experiment number")
    };

    ensure!(number != 0, "please enter a number other than 0 for the experiment number");
    ensure!(args.next().is_none(), "the input is too long");
    std::env::set_current_dir("../run")?;

    #[inline]
    fn run(python: Python, number: u32) -> Result<()> {
        python.import("sys")?.getattr("path")?.call_method1("append", (".",))?;
        python.import("depla")?.call_method0(&format!("Experiment{}", number))?.call_method0("run")?;
        Result::Ok(())
    }

    Python::with_gil(|python| run(python, number))?;
    Result::Ok(())
}
