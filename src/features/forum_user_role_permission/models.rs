use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::{forum::models::Model as ForumModel, permission::models::Model as PermissionModel, role::models::Model as RoleModel, user::models::Model as UserModel}};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_forum_user_role_permission: i32,
    pub id_forum: Option<i32>,
    pub id_user: Option<i32>,
    pub id_role: Option<i32>,
    pub id_permission: Option<i32>
}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_forum_user_role_permission: 0,
            id_forum: None,
            id_user: None,
            id_role: None,
            id_permission: None
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_forum_user_role_permission".to_string(),
                    ColumnMetadata {
                        column_name: "id_forum_user_role_permission".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "id_forum".to_string(),
                    ColumnMetadata {
                        column_name: "id_forum".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );

                columns.insert(
                    "id_user".to_string(),
                    ColumnMetadata {
                        column_name: "id_user".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );

                columns.insert(
                    "id_role".to_string(),
                    ColumnMetadata {
                        column_name: "id_role".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );

                columns.insert(
                    "id_permission".to_string(),
                    ColumnMetadata {
                        column_name: "id_permission".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );

                let mut relationships = HashMap::new();
                relationships.insert(
                    "Forum".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: ForumModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_forum".to_string()],
                        related_keys: vec!["id_forum".to_string()],
                    },
                );

                relationships.insert(
                    "User".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: UserModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_user".to_string()],
                        related_keys: vec!["id_user".to_string()],
                    },
                );

                relationships.insert(
                    "Role".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: RoleModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_role".to_string()],
                        related_keys: vec!["id_role".to_string()],
                    },
                );

                relationships.insert(
                    "Permission".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: PermissionModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_permission".to_string()],
                        related_keys: vec!["id_permission".to_string()]
                    },
                );

                EntityDescription {
                    table_name: "ForumUserRolePermission".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
