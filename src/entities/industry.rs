//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "industry")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::donar::Entity")]
    Donar,
}

impl Related<super::donar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Donar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
