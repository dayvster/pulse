use users::{get_current_uid, get_user_by_uid};

/// Returns the current user's username as a String, or UID if not found
pub fn get_current_username() -> String {
    let uid = get_current_uid();
    if let Some(user) = get_user_by_uid(uid) {
        if let Some(name) = user.name().to_str() {
            return name.to_string();
        }
    }
    uid.to_string()
}
