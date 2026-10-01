use rusqlite_migration::{Migrations, M};

pub fn migrations() -> Migrations<'static> {
    Migrations::new(vec![
        M::up(include_str!("migrations/0001_init.sql")),
        M::up(include_str!("migrations/0002_run_labels_and_scoped_view.sql")),
        M::up(include_str!("migrations/0003_run_dates_and_sectors.sql")),
        M::up(include_str!("migrations/0004_broker_orders.sql")),
    ])
}
