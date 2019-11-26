use bson;
use rocket_contrib::json::JsonValue;

pub fn validateObjectId(id: String) -> bool {
    match bson::oid::ObjectId::with_string(&id) {
        Ok(_) => {return true},
        Err(_) => {return false}
    }
}

pub fn generateErrorJson(errorMessage: String, status: usize) -> JsonValue {
    return json!({
        "code": status,
        "success": false,
        "data": {},
        "error": errorMessage
    });
}