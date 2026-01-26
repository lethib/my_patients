# Authorization Pattern - Rust Implementation

A fluent, composable authorization system ported from Rails to Rust. This pattern provides compile-time safety while maintaining an elegant, chainable API for authorization checks.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Implementation](#implementation)
- [Usage Examples](#usage-examples)
- [Web Framework Integration](#web-framework-integration)
- [Extending the System](#extending-the-system)

## Overview

This authorization system provides:

- **Fluent API**: Chain authorization checks naturally
- **Type Safety**: Compile-time guarantees
- **Composability**: Build complex checks from simple ones
- **Enforced Checks**: Must explicitly complete authorization
- **Short-circuiting**: Stops at first failure

### Example

```rust
auth.context
    .authorize()
    .authenticated_user()
    .user_owns_resource(&project)
    .required()?;
```

## Architecture

The system consists of three main components:

### 1. AuthContext

Holds authentication state and validates JWT tokens.

```rust
pub struct AuthContext {
    auth_token: Option<String>,
    current_user: Option<User>,
    authorized: bool,
    complete: bool,
    error: Option<AuthError>,
}
```

**Responsibilities:**
- Parse and validate authentication headers
- Decode JWT tokens
- Fetch and verify users
- Track authorization state
- Enforce single completion

### 2. AuthStatement

Builder pattern for composable authorization checks.

```rust
pub struct AuthStatement<'a> {
    auth_context: &'a mut AuthContext,
    is_empty: bool,
    ok_so_far: bool,
    error: Option<AuthError>,
}
```

**Responsibilities:**
- Chain multiple authorization checks
- Short-circuit on first failure
- Provide terminal methods (`required()`, `run_complete()`)
- Store intermediate errors

### 3. AuthError

Error types for authentication and authorization failures.

```rust
pub enum AuthError {
    NotAuthenticated,
    InvalidToken,
    UserNotFound,
    DeactivatedUser,
    AccessDenied { resource: String },
    AlreadyComplete,
}
```

## Implementation

### Core Types

```rust
// auth_error.rs
use std::fmt;

#[derive(Debug, Clone)]
pub enum AuthError {
    NotAuthenticated,
    InvalidToken,
    UserNotFound,
    InvalidSessionToken,
    DeactivatedUser,
    AccessDenied { resource: String },
    AlreadyComplete,
    Custom(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::NotAuthenticated => write!(f, "Not authenticated"),
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::InvalidSessionToken => write!(f, "Invalid session token"),
            AuthError::DeactivatedUser => write!(f, "User account is deactivated"),
            AuthError::AccessDenied { resource } => {
                write!(f, "Access denied to resource: {}", resource)
            }
            AuthError::AlreadyComplete => write!(f, "Authorization already completed"),
            AuthError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AuthError {}
```

### AuthContext Implementation

```rust
// auth_context.rs
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct TokenClaims {
    user_id: i64,
    session_token: String,
    exp: usize,
}

pub struct AuthContext {
    auth_token: Option<String>,
    current_user: Option<User>,
    authorized: bool,
    complete: bool,
    error: Option<AuthError>,
}

impl AuthContext {
    /// Create a new AuthContext from an optional Authorization header
    pub fn new(auth_header: Option<&str>) -> Self {
        let (current_user, error) = match auth_header {
            Some(header) => Self::validate_auth_header(header),
            None => (None, None),
        };

        Self {
            auth_token: auth_header.map(|h| {
                h.split_whitespace()
                    .last()
                    .unwrap_or_default()
                    .to_string()
            }),
            current_user,
            authorized: false,
            complete: false,
            error,
        }
    }

    /// Start building an authorization statement
    pub fn authorize(&mut self) -> AuthStatement {
        AuthStatement::new(self)
    }

    /// Validate the Authorization header and extract user
    fn validate_auth_header(auth_header: &str) -> (Option<User>, Option<AuthError>) {
        let token = match auth_header.split_whitespace().last() {
            Some(t) if !t.is_empty() => t,
            _ => return (None, Some(AuthError::NotAuthenticated)),
        };

        // Decode JWT token
        let claims = match decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data.claims,
            Err(_) => return (None, Some(AuthError::InvalidToken)),
        };

        // Fetch user from database
        let user = match fetch_user_by_id(claims.user_id) {
            Some(u) => u,
            None => return (None, Some(AuthError::UserNotFound)),
        };

        // Validate session token
        if user.session_token != claims.session_token {
            return (None, Some(AuthError::InvalidSessionToken));
        }

        (Some(user), None)
    }

    /// Mark authorization as successful (internal)
    pub(crate) fn authorized(&mut self) {
        if self.complete {
            panic!("auth_context_already_complete");
        }
        self.authorized = true;
    }

    /// Mark authorization as failed (internal)
    pub(crate) fn not_authorized(&mut self, error: AuthError) {
        if self.complete {
            panic!("auth_context_already_complete");
        }
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    /// Complete the authorization check (internal)
    pub(crate) fn mark_complete(&mut self) -> Result<(), AuthError> {
        if self.complete {
            panic!("auth_context_already_complete");
        }
        self.complete = true;

        if self.authorized {
            Ok(())
        } else {
            Err(self.error.take().unwrap_or(AuthError::AccessDenied {
                resource: "unknown".to_string(),
            }))
        }
    }

    /// Get the current authenticated user
    pub fn current_user(&self) -> Option<&User> {
        self.current_user.as_ref()
    }

    /// Get the current authenticated user (mutable)
    pub fn current_user_mut(&mut self) -> Option<&mut User> {
        self.current_user.as_mut()
    }

    /// Check if authorization is complete
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Check if authorized
    pub fn is_authorized(&self) -> bool {
        self.authorized
    }
}
```

### AuthStatement Implementation

```rust
// auth_statement.rs
pub struct AuthStatement<'a> {
    auth_context: &'a mut AuthContext,
    is_empty: bool,
    ok_so_far: bool,
    error: Option<AuthError>,
}

impl<'a> AuthStatement<'a> {
    pub(crate) fn new(auth_context: &'a mut AuthContext) -> Self {
        Self {
            auth_context,
            is_empty: true,
            ok_so_far: true,
            error: None,
        }
    }

    /// Terminal method - enforces authorization or returns error
    /// Use this when authorization MUST succeed
    pub fn required(mut self) -> Result<(), AuthError> {
        if self.is_empty {
            panic!("invalid_authorize_statement: no checks were performed");
        }

        if !self.ok_so_far {
            self.auth_context
                .not_authorized(self.error.take().unwrap());
        }

        self.auth_context.mark_complete()
    }

    /// Terminal method - runs check and returns whether it succeeded
    /// Use this when you want to branch based on authorization
    pub fn run(mut self) -> bool {
        if self.is_empty {
            panic!("invalid_authorize_statement: no checks were performed");
        }

        if self.ok_so_far {
            self.auth_context.authorized();
        } else {
            self.auth_context
                .not_authorized(self.error.take().unwrap());
        }

        self.ok_so_far
    }

    /// Terminal method - runs check, completes, and returns result
    pub fn run_complete(mut self) -> Result<bool, AuthError> {
        let result = self.run();
        self.auth_context.mark_complete()?;
        Ok(result)
    }

    // =========================================================================
    // Authorization Check Methods
    // =========================================================================

    /// Allow non-authenticated users
    pub fn non_authenticated_user(self) -> Self {
        self.check(|| Ok(true))
    }

    /// Require authenticated user
    pub fn authenticated_user(self) -> Self {
        self.check(|| {
            match self.auth_context.current_user() {
                None => Err(AuthError::NotAuthenticated),
                Some(user) if user.is_deactivated => Err(AuthError::DeactivatedUser),
                Some(_) => Ok(true),
            }
        })
    }

    /// Require user to be an admin
    pub fn admin_user(self) -> Self {
        let this = self.authenticated_user();
        this.check(|| {
            match this.auth_context.current_user() {
                Some(user) if user.is_admin => Ok(true),
                _ => Err(AuthError::AccessDenied {
                    resource: "admin".to_string(),
                }),
            }
        })
    }

    /// Require user to own the specified resource
    pub fn user_owns_resource<T: Resource>(self, resource: &T) -> Self {
        let this = self.authenticated_user();
        this.check(|| {
            match this.auth_context.current_user() {
                Some(user) if resource.is_owned_by(user.id) => Ok(true),
                _ => Err(AuthError::AccessDenied {
                    resource: resource.resource_name(),
                }),
            }
        })
    }

    /// Require user to have read access to resource
    pub fn can_read_resource<T: Resource>(self, resource: &T) -> Self {
        let this = self.authenticated_user();
        this.check(|| {
            match this.auth_context.current_user() {
                Some(user) if resource.can_read(user.id) => Ok(true),
                _ => Err(AuthError::AccessDenied {
                    resource: resource.resource_name(),
                }),
            }
        })
    }

    /// Require user to have write access to resource
    pub fn can_write_resource<T: Resource>(self, resource: &T) -> Self {
        let this = self.authenticated_user();
        this.check(|| {
            match this.auth_context.current_user() {
                Some(user) if resource.can_write(user.id) => Ok(true),
                _ => Err(AuthError::AccessDenied {
                    resource: resource.resource_name(),
                }),
            }
        })
    }

    /// Custom check with a predicate function
    pub fn custom_check<F>(self, predicate: F, error: AuthError) -> Self
    where
        F: FnOnce() -> bool,
    {
        self.check(|| if predicate() { Ok(true) } else { Err(error) })
    }

    // =========================================================================
    // Internal Check Method
    // =========================================================================

    /// Internal method to run a check function and update state
    /// Short-circuits if previous check already failed
    fn check<F>(mut self, check_fn: F) -> Self
    where
        F: FnOnce() -> Result<bool, AuthError>,
    {
        self.is_empty = false;

        // Short-circuit: don't run if already failed
        if !self.ok_so_far {
            return self;
        }

        match check_fn() {
            Ok(true) => self,
            Ok(false) => {
                self.ok_so_far = false;
                self
            }
            Err(err) => {
                self.ok_so_far = false;
                self.error = Some(err);
                self
            }
        }
    }
}
```

### Supporting Types

```rust
// user.rs
#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub session_token: String,
    pub is_deactivated: bool,
    pub is_admin: bool,
}

// resource.rs
pub trait Resource {
    /// Check if user owns this resource
    fn is_owned_by(&self, user_id: i64) -> bool;

    /// Check if user can read this resource
    fn can_read(&self, user_id: i64) -> bool {
        self.is_owned_by(user_id)
    }

    /// Check if user can write this resource
    fn can_write(&self, user_id: i64) -> bool {
        self.is_owned_by(user_id)
    }

    /// Get resource name for error messages
    fn resource_name(&self) -> String;
}

// Example resource implementation
#[derive(Debug)]
pub struct Project {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
}

impl Resource for Project {
    fn is_owned_by(&self, user_id: i64) -> bool {
        self.owner_id == user_id
    }

    fn resource_name(&self) -> String {
        format!("Project({})", self.id)
    }
}
```

## Usage Examples

### Basic Authentication

```rust
// Allow only authenticated users
auth.context
    .authorize()
    .authenticated_user()
    .required()?;
```

### Resource Ownership

```rust
// Require user owns the project
let project = fetch_project(project_id)?;

auth.context
    .authorize()
    .authenticated_user()
    .user_owns_resource(&project)
    .required()?;
```

### Admin Access

```rust
// Require admin user
auth.context
    .authorize()
    .admin_user()
    .required()?;
```

### Conditional Authorization

```rust
// Branch based on authorization result
let is_owner = auth.context
    .authorize()
    .authenticated_user()
    .user_owns_resource(&project)
    .run();

if is_owner {
    // Show edit options
} else {
    // Show read-only view
}

// Don't forget to complete!
auth.context.mark_complete()?;
```

### Complex Checks

```rust
// Chain multiple checks
auth.context
    .authorize()
    .authenticated_user()
    .admin_user()
    .custom_check(
        || project.is_active,
        AuthError::Custom("Project is archived".to_string())
    )
    .required()?;
```

## Web Framework Integration

### Axum

```rust
use axum::{
    extract::{FromRequestParts, Path, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, Router,
};
use std::sync::Arc;

// Extractor for authentication
pub struct Auth {
    pub context: AuthContext,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        Ok(Auth {
            context: AuthContext::new(auth_header),
        })
    }
}

// Convert AuthError to HTTP response
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::NotAuthenticated => {
                (StatusCode::UNAUTHORIZED, "Not authenticated")
            }
            AuthError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            AuthError::DeactivatedUser => {
                (StatusCode::FORBIDDEN, "Account deactivated")
            }
            AuthError::AccessDenied { resource } => {
                (StatusCode::FORBIDDEN, format!("Access denied: {}", resource))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string()),
        };

        (status, message.to_string()).into_response()
    }
}

// Example handlers
async fn get_project(
    mut auth: Auth,
    Path(project_id): Path<i64>,
) -> Result<Json<Project>, AuthError> {
    let project = fetch_project(project_id)?;

    auth.context
        .authorize()
        .authenticated_user()
        .user_owns_resource(&project)
        .required()?;

    Ok(Json(project))
}

async fn list_projects(
    mut auth: Auth,
) -> Result<Json<Vec<Project>>, AuthError> {
    auth.context
        .authorize()
        .authenticated_user()
        .required()?;

    let user = auth.context.current_user().unwrap();
    let projects = fetch_user_projects(user.id)?;

    Ok(Json(projects))
}

async fn admin_endpoint(
    mut auth: Auth,
) -> Result<&'static str, AuthError> {
    auth.context
        .authorize()
        .admin_user()
        .required()?;

    Ok("Admin data")
}
```

### Actix-web

```rust
use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorUnauthorized},
    web, Error, FromRequest, HttpRequest, HttpResponse,
};
use std::future::{ready, Ready};

pub struct Auth {
    pub context: AuthContext,
}

impl FromRequest for Auth {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        ready(Ok(Auth {
            context: AuthContext::new(auth_header),
        }))
    }
}

// Convert AuthError to Actix error
impl From<AuthError> for Error {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::NotAuthenticated | AuthError::InvalidToken => {
                ErrorUnauthorized(err.to_string())
            }
            AuthError::AccessDenied { .. } => ErrorForbidden(err.to_string()),
            _ => actix_web::error::ErrorInternalServerError(err.to_string()),
        }
    }
}

// Example handler
async fn get_project(
    mut auth: Auth,
    path: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    let project_id = path.into_inner();
    let project = fetch_project(project_id)?;

    auth.context
        .authorize()
        .authenticated_user()
        .user_owns_resource(&project)
        .required()?;

    Ok(HttpResponse::Ok().json(project))
}
```

## Extending the System

### Adding Custom Checks

```rust
impl<'a> AuthStatement<'a> {
    /// Check if user is a premium subscriber
    pub fn premium_user(self) -> Self {
        let this = self.authenticated_user();
        this.check(|| {
            match this.auth_context.current_user() {
                Some(user) if user.is_premium => Ok(true),
                _ => Err(AuthError::Custom("Premium subscription required".to_string())),
            }
        })
    }

    /// Check if user has verified email
    pub fn verified_email(self) -> Self {
        let this = self.authenticated_user();
        this.check(|| {
            match this.auth_context.current_user() {
                Some(user) if user.email_verified => Ok(true),
                _ => Err(AuthError::Custom("Email verification required".to_string())),
            }
        })
    }

    /// Check if user is in specific organization
    pub fn member_of_org(self, org_id: i64) -> Self {
        let this = self.authenticated_user();
        this.check(|| {
            match this.auth_context.current_user() {
                Some(user) if is_org_member(user.id, org_id) => Ok(true),
                _ => Err(AuthError::AccessDenied {
                    resource: format!("Organization({})", org_id),
                }),
            }
        })
    }
}
```

### Adding Resource Permissions

```rust
pub trait Resource {
    fn is_owned_by(&self, user_id: i64) -> bool;
    fn can_read(&self, user_id: i64) -> bool;
    fn can_write(&self, user_id: i64) -> bool;
    fn resource_name(&self) -> String;
}

pub struct Document {
    pub id: i64,
    pub owner_id: i64,
    pub shared_with: Vec<i64>,
}

impl Resource for Document {
    fn is_owned_by(&self, user_id: i64) -> bool {
        self.owner_id == user_id
    }

    fn can_read(&self, user_id: i64) -> bool {
        self.is_owned_by(user_id) || self.shared_with.contains(&user_id)
    }

    fn can_write(&self, user_id: i64) -> bool {
        self.is_owned_by(user_id)
    }

    fn resource_name(&self) -> String {
        format!("Document({})", self.id)
    }
}
```

### Multiple Token Types

```rust
impl AuthContext {
    pub fn new_with_api_key(api_key: Option<&str>) -> Self {
        let (current_user, error) = match api_key {
            Some(key) => Self::validate_api_key(key),
            None => (None, None),
        };

        Self {
            auth_token: api_key.map(|k| k.to_string()),
            current_user,
            authorized: false,
            complete: false,
            error,
        }
    }

    fn validate_api_key(api_key: &str) -> (Option<User>, Option<AuthError>) {
        match fetch_user_by_api_key(api_key) {
            Some(user) => (Some(user), None),
            None => (None, Some(AuthError::InvalidToken)),
        }
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticated_user_required() {
        let mut ctx = AuthContext::new(None);
        let result = ctx.authorize().authenticated_user().required();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuthError::NotAuthenticated));
    }

    #[test]
    fn test_valid_authentication() {
        let token = create_test_token(user_id: 1);
        let auth_header = format!("Bearer {}", token);

        let mut ctx = AuthContext::new(Some(&auth_header));
        let result = ctx.authorize().authenticated_user().required();

        assert!(result.is_ok());
        assert_eq!(ctx.current_user().unwrap().id, 1);
    }

    #[test]
    fn test_resource_ownership() {
        let token = create_test_token(user_id: 1);
        let auth_header = format!("Bearer {}", token);

        let mut ctx = AuthContext::new(Some(&auth_header));
        let project = Project { id: 1, owner_id: 1, name: "Test".into() };

        let result = ctx
            .authorize()
            .authenticated_user()
            .user_owns_resource(&project)
            .required();

        assert!(result.is_ok());
    }

    #[test]
    fn test_short_circuit_on_failure() {
        let mut ctx = AuthContext::new(None);
        let mut check_called = false;

        ctx.authorize()
            .authenticated_user()  // This will fail
            .custom_check(|| {
                check_called = true;
                true
            }, AuthError::Custom("test".into()))
            .required()
            .ok();

        // Second check should not be called due to short-circuit
        assert!(!check_called);
    }

    #[test]
    #[should_panic(expected = "auth_context_already_complete")]
    fn test_cannot_authorize_twice() {
        let mut ctx = AuthContext::new(None);
        ctx.authorize().non_authenticated_user().required().ok();
        ctx.authorize().non_authenticated_user().required().ok();
    }
}
```

## Best Practices

1. **Always call a terminal method** - Every authorization chain must end with `.required()`, `.run()`, or `.run_complete()`

2. **Order matters** - Put cheaper checks first (e.g., `authenticated_user()` before database lookups)

3. **Use specific errors** - Create custom error types for better error messages

4. **Don't reuse AuthContext** - Create a new context per request

5. **Test authorization** - Write tests for both success and failure cases

6. **Document custom checks** - Add clear doc comments to custom authorization methods

## Performance Considerations

- **Short-circuiting**: Checks stop at first failure, so order expensive checks last
- **Database queries**: Cache user lookups if needed
- **Token validation**: Consider caching decoded tokens with expiration
- **Resource checks**: Implement efficient database queries in `Resource` trait methods

## Security Notes

- Always validate JWT expiration times
- Use secure secret keys for JWT signing
- Implement rate limiting on authentication endpoints
- Log authentication failures for security monitoring
- Consider adding IP-based restrictions
- Rotate session tokens on password change
- Implement proper CSRF protection in web applications

---

## Comparison with Rails Version

### Similarities

- Fluent chaining API
- Short-circuiting on failure
- Enforced completion
- Composable checks

### Differences

- **Type safety**: Compile-time guarantees vs runtime checks
- **Error handling**: `Result<T, E>` vs exceptions
- **Ownership**: Rust's borrow checker requires `&mut` references
- **Panics**: Used for programming errors (like Rails' `raise`)
- **Resource trait**: Explicit trait vs duck typing

### Migration Path

If migrating from Rails:

1. Map each Rails check method to a Rust equivalent
2. Convert `raise` errors to `AuthError` variants
3. Replace dynamic typing with `Resource` trait
4. Update tests to use `Result` assertions
5. Add type annotations for resources

---

**License**: MIT
**Author**: Thibault
**Version**: 1.0.0
