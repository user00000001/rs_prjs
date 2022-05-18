use std::env;
use bytes::Bytes;
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use tokio::sync::Mutex as TMutex;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

struct CanIncrement {
    mutex: Mutex<i32>,
}

impl CanIncrement {
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(10);
    tokio::spawn(async move {
        increment_and_do_stuff(&mutex).await;
    });
    let mutex = CanIncrement { mutex: Mutex::new(10) };
    tokio::spawn(async move {
        increment_and_do_stuff1(&mutex).await;
    });
    let tmutex = TMutex::new(10);
    tokio::spawn(async move {
        increment_and_do_stuff2(&tmutex).await;
    });


    let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();
    println!("Listening");
    let sharded = env::var("SHARDED").is_err();
    let db = Arc::new(Mutex::new(HashMap::new()));
    if !sharded {
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let db = db.clone();
            // println!("Accepted");
            tokio::spawn(async move {
                process(socket, db).await;
            });
        }
    } else {
        let sharded_db = new_sharded_db(50); // for i in {0..1000}; do mini-redis-cli --port 6378 set $i $(($i*$i)) & done
        loop {                               // for i in {0..1000}; do mini-redis-cli --port 6378 get $i & done
            let (socket, _) = listener.accept().await.unwrap();
            let sharded_db = sharded_db.clone();
            // println!("Accepted");
            tokio::spawn(async move {
                process1(socket, sharded_db).await;
            });
        }
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let rsp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                println!("Set: {} => {:?}", cmd.key().to_string(), cmd.value());
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    println!("GET: {} => {:?}", cmd.key(), value);
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&rsp).await.unwrap();
    }
}

async fn process1(socket: TcpStream, sharded_db: ShardedDb) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let rsp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = sharded_db[my_hash(cmd.key()) % sharded_db.len()].lock().unwrap();
                println!("Set: {} => {:?}", cmd.key().to_string(), cmd.value());
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = sharded_db[my_hash(cmd.key()) % sharded_db.len()].lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    println!("GET: {} => {:?}", cmd.key(), value);
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&rsp).await.unwrap();
    }
}

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

fn my_hash<T>(obj: T) -> usize
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish() as usize
}

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    // let mut lock: MutexGuard<i32> = mutex.lock().unwrap(); // ERROR: future created by async block is not `Send`
    // *lock += 1;

    // drop(lock); // not working: bases on scope information only, support explicitly dropping it in the future, but not now

    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}

async fn increment_and_do_stuff1(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}

async fn increment_and_do_stuff2(tmutex: &TMutex<i32>) {
    let mut lock = tmutex.lock().await;
    *lock += 1;

    do_something_async().await;
}

async fn do_something_async() {
    println!("Do something async");
}