//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "t_sc")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub f_sno: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub f_cno: String,
    pub f_grade: Option<i8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::t_course::Entity",
        from = "Column::FCno",
        to = "super::t_course::Column::FCno",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TCourse,
    #[sea_orm(
        belongs_to = "super::t_student::Entity",
        from = "Column::FSno",
        to = "super::t_student::Column::FSno",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TStudent,
}

impl Related<super::t_course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TCourse.def()
    }
}

impl Related<super::t_student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TStudent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
