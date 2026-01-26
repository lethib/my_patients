use crate::{
  auth::{context::AuthContext, resource::Resource},
  models::my_errors::{authentication_error::AuthenticationError, MyErrors},
};

pub struct AuthStatement {
  auth_context: AuthContext,
  is_empty: bool,
  ok_so_far: bool,
  error: Option<MyErrors>,
}

impl AuthStatement {
  pub(super) fn new(auth_context: AuthContext) -> Self {
    Self {
      auth_context,
      is_empty: true,
      ok_so_far: true,
      error: None,
    }
  }

  pub fn run_complete(mut self) -> Result<(), MyErrors> {
    if self.is_empty {
      panic!("invalid_authorize_statement: no checks performed")
    }

    if self.ok_so_far {
      self.auth_context.authorized();
    } else {
      self.auth_context.not_authorized(self.error.take());
    }

    self.auth_context.complete()
  }

  pub fn non_authenticated_user(self) -> Self {
    self.check(|_| true, None)
  }

  pub fn authenticated_user(self) -> Self {
    self.check(
      |s| s.auth_context.current_user.is_some(),
      Some(AuthenticationError::INVALID_CREDENTIALS()),
    )
  }

  pub async fn is_owning_resource<T: Resource>(self, resource: &T) -> Self {
    let is_owned = match &self.auth_context.current_user {
      Some(user) => resource.is_owned_by_user(user.0.id).await,
      None => false,
    };

    self.check(
      |_| is_owned,
      Some(AuthenticationError::ACCESS_DENIED(Some(
        resource.resource_name(),
      ))),
    )
  }

  #[allow(dead_code)]
  pub fn or<F>(mut self, check_fn: F) -> Self
  where
    F: FnOnce(Self) -> Self,
  {
    if self.ok_so_far {
      return self;
    }

    // checks needs to be fresh here (a successfull check can happen after a failed one)
    self.ok_so_far = true;
    self.error = None;

    check_fn(self)
  }

  pub fn check<F>(mut self, predicate: F, error: Option<MyErrors>) -> Self
  where
    F: FnOnce(&Self) -> bool,
  {
    self.is_empty = false;

    if !self.ok_so_far {
      return self;
    }

    if predicate(&self) {
      return self;
    } else {
      self.ok_so_far = false;
      self.error = error;
      return self;
    }
  }
}
