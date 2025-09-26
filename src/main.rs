use det::config::yaml;
use anyhow::Result;

fn main() -> Result<()> {
    let buf = std::fs::read_to_string("config.yaml")?;
    let cfg = yaml::parse_det_config(&buf)?; // cfg borrows from buf
    let sources = cfg.extract.sources.keys();
    println!("{:?}", sources);
    Ok(())
}
