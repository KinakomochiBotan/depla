use anyhow::{
    Result,
    bail
};

use pyo3::Python;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();

    let experiment = match args.next() {
        Option::Some(experiment) => experiment.replace(".", ""),
        Option::None => bail!("please enter an experiment number")
    };

    std::env::set_current_dir("run")?;

    #[inline]
    fn run(python: Python, experiment: &str) -> Result<()> {
        python.import("sys")?.getattr("path")?.call_method1("append", (".",))?;
        python.import("depla")?.call_method0(&format!("Experiment{}", experiment))?.call_method0("run")?;
        Result::Ok(())
    }

    Python::with_gil(|python| run(python, &experiment))?;
    Result::Ok(())
}
