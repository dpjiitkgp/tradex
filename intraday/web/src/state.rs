use backtest::JobProgress;
use core_types::RunId;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum JobStatus {
    Running { progress: Option<ProgressDto> },
    Done { run_id: RunId },
    Failed { error: String },
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProgressDto {
    pub current: usize,
    pub total: usize,
    pub message: String,
}

impl From<JobProgress> for ProgressDto {
    fn from(p: JobProgress) -> Self {
        Self {
            current: p.current,
            total: p.total,
            message: p.message,
        }
    }
}

/// In-memory registry of triggered-backtest jobs — deliberately not
/// persisted: a job is just "is this particular trigger still running,"
/// the actual result (the run itself) lives in SQLite like everything
/// else once the job finishes.
#[derive(Default, Clone)]
pub struct JobRegistry(Arc<Mutex<HashMap<Uuid, JobStatus>>>);

impl JobRegistry {
    pub fn insert_running(&self) -> Uuid {
        let id = Uuid::new_v4();
        self.0.lock().unwrap().insert(id, JobStatus::Running { progress: None });
        id
    }

    pub fn update_progress(&self, id: Uuid, progress: JobProgress) {
        self.0.lock().unwrap().insert(
            id,
            JobStatus::Running {
                progress: Some(progress.into()),
            },
        );
    }

    pub fn finish_ok(&self, id: Uuid, run_id: RunId) {
        self.0.lock().unwrap().insert(id, JobStatus::Done { run_id });
    }

    pub fn finish_err(&self, id: Uuid, error: String) {
        self.0.lock().unwrap().insert(id, JobStatus::Failed { error });
    }

    pub fn get(&self, id: Uuid) -> Option<JobStatus> {
        self.0.lock().unwrap().get(&id).cloned()
    }
}

/// Whether the one-time (per process) sector sync has run yet — cheap
/// guard so `/api/universe/*` doesn't re-fetch NSE's constituent CSV and
/// re-hit Kite's instrument master on every request.
#[derive(Default, Clone)]
pub struct SectorSyncState(Arc<Mutex<bool>>);

impl SectorSyncState {
    pub fn is_done(&self) -> bool {
        *self.0.lock().unwrap()
    }
    pub fn mark_done(&self) {
        *self.0.lock().unwrap() = true;
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db_path: PathBuf,
    pub config_path: PathBuf,
    pub credentials_path: PathBuf,
    pub jobs: JobRegistry,
    pub sector_sync: SectorSyncState,
}

impl AppState {
    pub fn open_storage(&self) -> anyhow::Result<storage::SqliteStorage> {
        storage::SqliteStorage::open(&self.db_path)
    }
}
