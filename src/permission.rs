//! Permission helpers for Appwrite

/// Permission helper functions
pub struct Permission;

impl Permission {
    /// Create a read permission for a role
    pub fn read(role: impl AsRef<str>) -> String {
        format!(r#"read("{}")"#, role.as_ref())
    }

    /// Create a write permission for a role
    pub fn write(role: impl AsRef<str>) -> String {
        format!(r#"write("{}")"#, role.as_ref())
    }

    /// Create a create permission for a role
    pub fn create(role: impl AsRef<str>) -> String {
        format!(r#"create("{}")"#, role.as_ref())
    }

    /// Create an update permission for a role
    pub fn update(role: impl AsRef<str>) -> String {
        format!(r#"update("{}")"#, role.as_ref())
    }

    /// Create a delete permission for a role
    pub fn delete(role: impl AsRef<str>) -> String {
        format!(r#"delete("{}")"#, role.as_ref())
    }
}

/// Role helper functions
pub struct Role;

impl Role {
    /// Any role
    pub fn any() -> String {
        "any".to_string()
    }

    /// User role with specific user ID
    pub fn user(id: impl AsRef<str>) -> String {
        format!("user:{}", id.as_ref())
    }

    /// Users role (any authenticated user)
    pub fn users() -> String {
        "users".to_string()
    }

    /// Guests role (unauthenticated users)
    pub fn guests() -> String {
        "guests".to_string()
    }

    /// Team role with specific team ID
    pub fn team(id: impl AsRef<str>) -> String {
        format!("team:{}", id.as_ref())
    }

    /// Team role with specific team ID and role
    pub fn team_with_role(id: impl AsRef<str>, role: impl AsRef<str>) -> String {
        format!("team:{}:{}", id.as_ref(), role.as_ref())
    }

    /// Member role for a team
    pub fn member(id: impl AsRef<str>) -> String {
        format!("member:{}", id.as_ref())
    }

    /// Label role
    pub fn label(name: impl AsRef<str>) -> String {
        format!("label:{}", name.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permissions() {
        assert_eq!(Permission::read("user:123"), r#"read("user:123")"#);
        assert_eq!(Permission::write("team:456"), r#"write("team:456")"#);
        assert_eq!(Permission::create("any"), r#"create("any")"#);
        assert_eq!(Permission::update("users"), r#"update("users")"#);
        assert_eq!(Permission::delete("guests"), r#"delete("guests")"#);
    }

    #[test]
    fn test_roles() {
        assert_eq!(Role::any(), "any");
        assert_eq!(Role::user("123"), "user:123");
        assert_eq!(Role::users(), "users");
        assert_eq!(Role::guests(), "guests");
        assert_eq!(Role::team("456"), "team:456");
        assert_eq!(Role::team_with_role("456", "admin"), "team:456:admin");
        assert_eq!(Role::member("789"), "member:789");
        assert_eq!(Role::label("vip"), "label:vip");
    }
}