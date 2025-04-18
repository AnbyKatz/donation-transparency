//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "donation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub year: String,
    pub amount: i64,
    pub branch_id: i32,
    pub donar_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::branch::Entity",
        from = "Column::BranchId",
        to = "super::branch::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Branch,
    #[sea_orm(
        belongs_to = "super::donar::Entity",
        from = "Column::DonarId",
        to = "super::donar::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Donar,
}

impl Related<super::branch::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Branch.def()
    }
}

impl Related<super::donar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Donar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
