use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub struct DbPool(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbPool {
    type Context = SyncContext<Self>;
}

pub type DbAddr = Addr<DbPool>;

pub fn init_db_pool() -> DbAddr {
    let db_url = dotenv::var("DATABASE_URL").unwrap_or_else(|_| "DATABASE_URL must be set".into());
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let cpu_num = num_cpus::get();
    let pool_num = std::cmp::max(10, cpu_num * 2 + 1) as u32;
    // p_num subject to c_num??
    let conn = Pool::builder()
        .max_size(pool_num)
        .build(manager)
        .expect("Failed to create pool.");

    SyncArbiter::start(cpu_num * 2 + 1, move || DbPool(conn.clone()))
}
