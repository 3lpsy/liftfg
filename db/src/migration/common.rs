use sea_orm_migration::prelude::*;
pub trait TableWithTimestamps {
    fn add_timestamps(&mut self) -> &mut Self;
}
impl TableWithTimestamps for TableCreateStatement {
    fn add_timestamps(&mut self) -> &mut Self {
        self.col(
            ColumnDef::new(Timestamp::CreatedAt)
                .timestamp()
                .not_null()
                .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
        )
        .col(
            ColumnDef::new(Timestamp::UpdatedAt)
                .timestamp()
                .not_null()
                .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
        )
    }
}

// Create an enum for the timestamp column names
#[derive(DeriveIden)]
enum Timestamp {
    CreatedAt,
    UpdatedAt,
}
