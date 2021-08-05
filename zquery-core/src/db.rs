use std::{thread};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::atomic::AtomicBool;
use rusqlite::{Connection, NO_PARAMS};
use std::path::Path;
use super::errors::ZqError;

#[derive(Debug)]
pub enum DbMsg{
    Stop,
}

type DbSender = Sender<DbMsg>;

pub struct Db {
    _db_conn : Connection,
    _tx : DbSender,
    worker : DbWorker,
}

struct DbWorker{
    handle : thread::JoinHandle<()>,
    _runnig : AtomicBool,

}

impl DbWorker {
    pub fn new(_rx : Receiver<DbMsg>) -> DbWorker {
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

    DbWorker{handle, _runnig : AtomicBool::new(true)}
    }

    pub fn wait(self) {
        self.handle.join().unwrap();
    }
}

impl Db {
    pub fn new() -> Result<Db, ZqError> {
        let conn = Connection::open(Path::new("tmpdb.db")).map_err(ZqError::Db)?;

        let (tx, rx) = channel::<DbMsg>();
        let worker = DbWorker::new(rx);
        let db = Db{_db_conn : conn, _tx : tx, worker: worker};

        Ok(db)
    }

    pub fn _get_sender(&self) -> DbSender {
       self._tx.clone() 
    }

    pub fn _db_version(&self) -> Result<String, ZqError> { 
        let version = self._db_conn.query_row("SELECT sqlite_version()", NO_PARAMS, |row| row.get(0)).map_err(ZqError::Db)?;
        Ok(version)
    }

    pub fn stop_worker(self) {
        self._tx.send(DbMsg::Stop).unwrap();
        self.worker.wait();
    }
}
