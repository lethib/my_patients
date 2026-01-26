use std::future::Future;

pub trait Resource {
  fn is_owned_by_user(&self, user_id: i32) -> impl Future<Output = bool>;

  fn resource_name(&self) -> String;
}
