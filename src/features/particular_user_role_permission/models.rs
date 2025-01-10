use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::{particular::models::ParticularModel, permission::models::PermissionModel, role::models::RoleModel, user::models::UserModel}};

#[derive(Serialize, Deserialize)]
pub struct ParticularUserRolePermissionModel {
    pub id_particular_user_role_permission: i32,
    pub id_particular: Option<i32>,
    pub id_user: Option<i32>,
    pub id_role: Option<i32>,
    pub id_permission: Option<i32>
}

impl Default for ParticularUserRolePermissionModel {
    fn default() -> Self {
        ParticularUserRolePermissionModel {
            id_particular_user_role_permission: 0,
            id_particular: None,
            id_user: None,
            id_role: None,
            id_permission: None
        }
    }
}

impl EntityMetadata for ParticularUserRolePermissionModel {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_particular_user_role_permission".to_string(),
                    ColumnMetadata {
                        column_name: "id_particular_user_role_permission".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "id_particular".to_string(),
                    ColumnMetadata {
                        column_name: "id_particular".to_string(),
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
                    "Particular".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: ParticularModel::metadata().clone(),
                        related_entity: "Particular".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_particular".to_string()],
                        related_keys: vec!["id_particular".to_string()],
                        navigation_property: "Particular".to_string(),
                    },
                );

                relationships.insert(
                    "User".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: UserModel::metadata().clone(),
                        related_entity: "User".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_user".to_string()],
                        related_keys: vec!["id_user".to_string()],
                        navigation_property: "User".to_string(),
                    },
                );

                relationships.insert(
                    "Role".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: RoleModel::metadata().clone(),
                        related_entity: "Role".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_role".to_string()],
                        related_keys: vec!["id_role".to_string()],
                        navigation_property: "Role".to_string(),
                    },
                );

                relationships.insert(
                    "Permission".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: PermissionModel::metadata().clone(),
                        related_entity: "Permission".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_permission".to_string()],
                        related_keys: vec!["id_permission".to_string()],
                        navigation_property: "Permission".to_string(),
                    },
                );

                EntityDescription {
                    table_name: "ParticularUserRolePermission".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
