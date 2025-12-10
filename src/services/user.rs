use crate::{
  initializers::get_services, models::_entities::user_business_informations::ActiveModel,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, IntoActiveModel, ModelTrait};

use crate::models::{
  _entities::user_business_informations, user_business_informations::CreateBusinessInfomation,
  users,
};

pub async fn save_business_information(
  params: &CreateBusinessInfomation,
  concerned_user: &users::Model,
) -> Result<(), DbErr> {
  let services = get_services();

  let business_info = concerned_user
    .find_related(user_business_informations::Entity)
    .one(&services.db)
    .await?;

  match business_info {
    Some(business_information) => {
      let mut business_information = business_information.into_active_model();

      business_information.rpps_number = Set(params.rpps_number.clone());
      business_information.siret_number = Set(params.siret_number.clone());
      business_information.adeli_number = Set(params.adeli_number.clone());

      business_information.update(&services.db).await?;
      Ok(())
    }
    None => {
      ActiveModel::create(&services.db, params, &concerned_user.id).await?;
      Ok(())
    }
  }
}

pub fn check_access_key(concerned_user: &users::Model, access_key: String) -> bool {
  match &concerned_user.access_key {
    None => false,
    Some(user_access_key) => *user_access_key == access_key,
  }
}
