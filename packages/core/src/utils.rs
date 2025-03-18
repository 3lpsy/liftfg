#[cfg(test)]
pub mod testutils {
    use crate::logging;
    use anyhow::{anyhow, Result};
    use ctor::ctor;
    use fgdb::db::{self, get_dbc};
    use sea_orm::DatabaseConnection;
    use std::fs;
    use std::sync::OnceLock;
    use uuid::Uuid;

    static INIT: OnceLock<()> = OnceLock::new();
    static DATA_DIR: OnceLock<std::path::PathBuf> = OnceLock::new();
    pub static LOGGING_HANDLES: OnceLock<(logging::LayersHandle, logging::FilterHandle)> =
        OnceLock::new();

    #[ctor]
    fn init_tests() {
        INIT.get_or_init(|| {
            let data_dir = fgutils::find_workspace_root()
                .unwrap()
                .join("appdata")
                .join("tests");
            if !data_dir.exists() {
                fs::create_dir_all(&data_dir).expect("Could not create testing data dir");
            }
            DATA_DIR.set(data_dir).unwrap();
            let handles = logging::init().unwrap();
            match LOGGING_HANDLES.set(handles) {
                Ok(_) => Ok(()),
                Err(_e) => Err(anyhow!("Failed to set logging handles on lock")),
            }
            .unwrap();
        });
    }

    pub fn data_dir(test_id: Uuid) -> std::path::PathBuf {
        DATA_DIR.get().unwrap().join(test_id.to_string())
    }

    async fn setup_test_db(should_migrate: bool) -> Result<(DatabaseConnection, Uuid)> {
        let test_id = Uuid::new_v4();
        let db_path = data_dir(test_id);

        if should_migrate {
            db::migrate(&db_path).await?;
        }
        // logging setup in ctor
        // though no file logging exists atm, requires reload

        let dbc = get_dbc(&db_path).await?;
        Ok((dbc, test_id))
    }

    pub async fn setup_test_db_full() -> Result<(DatabaseConnection, Uuid)> {
        setup_test_db(true).await
    }
}
