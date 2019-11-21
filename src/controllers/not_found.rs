use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn lookup() -> JsonValue {
  json!({
    "success": false,
    "code": 404,
    "data": "",
    "error": "Resource not found"
  })
}