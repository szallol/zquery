use super::errors::{Result, ZqError};
use rusqlite::{Connection, NO_PARAMS};
use std::fmt::Error;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, vec};

#[derive(Debug)]
pub enum DbMsg {
    Stop,
}

type DbSender = Sender<DbMsg>;

pub struct Db {
    db_conn: Connection,
    tx: DbSender,
    worker: DbWorker,
}

struct DbWorker {
    handle: thread::JoinHandle<()>,
    _runnig: AtomicBool,
}

impl DbWorker {
    pub fn new(_rx: Receiver<DbMsg>) -> DbWorker {
        let handle = thread::spawn(move || {
            println!("db worker start waiting");

            let mut runnig = true;
            while runnig {
                while let Ok(something) = _rx.try_recv() {
                    println!("db got something: {:?}", something);
                    match something {
                        DbMsg::Stop => runnig = false,
                    }
                }
            }
        });

        DbWorker {
            handle,
            _runnig: AtomicBool::new(true),
        }
    }

    pub fn wait(self) {
        self.handle.join().unwrap();
    }
}

impl Db {
    pub fn new() -> Result<Db> {
        let conn = Connection::open(Path::new("tmpdb.db")).map_err(ZqError::Db)?;

        let (tx, rx) = channel::<DbMsg>();
        let worker = DbWorker::new(rx);
        let db = Db {
            db_conn: conn,
            tx,
            worker: worker,
        };

        Ok(db)
    }

    pub fn _get_sender(&self) -> DbSender {
        self.tx.clone()
    }

    pub fn db_version(&self) -> Result<String> {
        let version = self
            .db_conn
            .query_row("SELECT sqlite_version()", NO_PARAMS, |row| row.get(0))
            .map_err(ZqError::Db)?;

        Ok(version)
    }

    pub fn stop_worker(self) {
        self.tx.send(DbMsg::Stop).unwrap();
        self.worker.wait();
    }

    pub fn execute_query(&self, query: &str) -> Result<Vec<String>> {
        let mut stmt = self.db_conn.prepare(query).map_err(ZqError::DbQuery)?;
        let rows = stmt.query(NO_PARAMS).map_err(ZqError::DbQuery)?;

        if let Some(names) = rows.column_names() {
            let names_string = names.iter().map(|x| x.to_string()).collect();
            return Ok(names_string);
        }

        Err(ZqError::DbGetColumnNames())
    }
}
