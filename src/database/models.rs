#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserWithoutPassword {
    pub id: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct UserWithPassword {
    pub id: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(serde::Serialize)]
pub struct Individual {
    pub id: String,
    pub name: String,
    pub image: String,
    pub birthday: String,
    pub generation: i32,
    pub is_alive: bool,
    pub death: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(serde::Serialize)]
pub struct Relationship {
    pub from_individual_id: String,
    pub to_individual_id: String,
    pub relationship_type: String,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "relationship_types", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipType {
    ParentSon,
    Marriage,
    Divorce,
}

impl RelationshipType {
    pub fn to_string(&self) -> String {
        match self {
            RelationshipType::ParentSon => "PARENT_SON".to_string(),
            RelationshipType::Marriage => "MARRIAGE".to_string(),
            RelationshipType::Divorce => "DIVORCE".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Result<RelationshipType, String> {
        match s {
            "PARENT_SON" => Ok(RelationshipType::ParentSon),
            "MARRIAGE" => Ok(RelationshipType::Marriage),
            "DIVORCE" => Ok(RelationshipType::Divorce),
            _ => Err("Invalid relationship type".to_string()),
        }
    }
}
