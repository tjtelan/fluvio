use structopt::StructOpt;

use crate::ClusterUninstaller;
use crate::cli::ClusterCliError;

#[derive(Debug, StructOpt)]
pub struct DeleteOpt {
    #[structopt(long, default_value = "default")]
    namespace: String,

    #[structopt(long, default_value = "fluvio")]
    name: String,

    /// don't wait for clean up
    #[structopt(long)]
    no_wait: bool,

    /// Remove local spu/sc(custom) fluvio installation
    #[structopt(long)]
    local: bool,

    #[structopt(long)]
    /// Remove fluvio system chart
    sys: bool,
}

impl DeleteOpt {
    pub async fn process(self) -> Result<(), ClusterCliError> {
        let uninstaller = ClusterUninstaller::new()
            .with_namespace(&self.namespace)
            .with_name(&self.name)
            .build()?;
        if self.sys {
            uninstaller.uninstall_sys().await?;
        } else if self.local {
            uninstaller.uninstall_local().await?;
        } else {
            uninstaller.uninstall().await?;
        }

        Ok(())
    }
}
