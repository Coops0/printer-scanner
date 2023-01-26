use std::process::ExitStatus;
use tokio::process::Command;
use anyhow::Result;

pub async fn print(printer_system_name: &str, file_path: &str) -> Result<ExitStatus>  {
      Ok(
              Command::new("lp")
                  .arg("-d")
                  .arg(printer_system_name)
                  .arg(file_path)
                  .spawn()?
                  .wait()
                  .await?
      )
}