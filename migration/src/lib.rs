#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250809_080459_remove_unnecessary_cols_from_users;
mod m20250820_151249_patients;
mod m20250820_152922_create_join_table_users_and_patients;
mod m20250902_193546_add_hashed_ssn_to_patients;
mod m20250907_000001_split_patient_name;
mod m20250910_203200_add_address_to_patients;
mod m20250912_161133_user_business_informations;
mod m20250912_195210_add_unique_constraint_user_business_info;
mod m20250913_125707_add_office_to_patient;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20220101_000001_users::Migration),
      Box::new(m20250809_080459_remove_unnecessary_cols_from_users::Migration),
      Box::new(m20250820_151249_patients::Migration),
      Box::new(m20250820_152922_create_join_table_users_and_patients::Migration),
      Box::new(m20250902_193546_add_hashed_ssn_to_patients::Migration),
      Box::new(m20250907_000001_split_patient_name::Migration),
      Box::new(m20250910_203200_add_address_to_patients::Migration),
      Box::new(m20250912_161133_user_business_informations::Migration),
      Box::new(m20250912_195210_add_unique_constraint_user_business_info::Migration),
      Box::new(m20250913_125707_add_office_to_patient::Migration),
      // inject-above (do not remove this comment)
    ]
  }
}
