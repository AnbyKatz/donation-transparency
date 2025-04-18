//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "donar")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub industry_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::donation::Entity")]
    Donation,
    #[sea_orm(
        belongs_to = "super::industry::Entity",
        from = "Column::IndustryId",
        to = "super::industry::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Industry,
}

impl Related<super::donation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Donation.def()
    }
}

impl Related<super::industry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Industry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
