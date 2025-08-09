use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        remove_column(m, "users", "api_key").await?;
        remove_column(m, "users", "reset_token").await?;
        remove_column(m, "users", "reset_sent_at").await?;
        remove_column(m, "users", "email_verification_token").await?;
        remove_column(m, "users", "email_verification_sent_at").await?;
        remove_column(m, "users", "email_verified_at").await?;
        remove_column(m, "users", "magic_link_token").await?;
        remove_column(m, "users", "magic_link_expiration").await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        add_column(m, "users", "api_key", ColType::StringUniq).await?;
        add_column(m, "users", "reset_token", ColType::StringNull).await?;
        add_column(
            m,
            "users",
            "reset_sent_at",
            ColType::TimestampWithTimeZoneNull,
        )
        .await?;
        add_column(m, "users", "email_verification_token", ColType::StringNull).await?;
        add_column(
            m,
            "users",
            "email_verification_sent_at",
            ColType::TimestampWithTimeZoneNull,
        )
        .await?;
        add_column(
            m,
            "users",
            "email_verified_at",
            ColType::TimestampWithTimeZoneNull,
        )
        .await?;
        add_column(m, "users", "magic_link_token", ColType::StringNull).await?;
        add_column(
            m,
            "users",
            "magic_link_expiration",
            ColType::TimestampWithTimeZoneNull,
        )
        .await?;
        Ok(())
    }
}
