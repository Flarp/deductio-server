use rocket_db_pools::diesel::{QueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use crate::schema::*;
use crate::model::KnowledgeGraph;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(KnowledgeGraph), table_name = topics)]

pub struct Topic {
    pub knowledge_graph_id: uuid::Uuid,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub subject: String,
    pub content: serde_json::Value
}