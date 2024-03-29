// @generated automatically by Diesel CLI.

diesel::table! {
    extensions (source, destination) {
        source -> Uuid,
        destination -> Uuid,
    }
}

diesel::table! {
    knowledge_graphs (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        owner -> Text,
    }
}

diesel::table! {
    progress (user_id, graph) {
        user_id -> Text,
        graph -> Uuid,
        user_progress -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    requirements (id) {
        source -> Int8,
        destination -> Int8,
        knowledge_graph_id -> Uuid,
        id -> Int8,
    }
}

diesel::table! {
    topics (id) {
        knowledge_graph_id -> Uuid,
        title -> Text,
        id -> Int8,
        subject -> Text,
        content -> Jsonb,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
    }
}

diesel::joinable!(progress -> knowledge_graphs (graph));
diesel::joinable!(progress -> users (user_id));
diesel::joinable!(requirements -> knowledge_graphs (knowledge_graph_id));
diesel::joinable!(topics -> knowledge_graphs (knowledge_graph_id));

diesel::allow_tables_to_appear_in_same_query!(
    extensions,
    knowledge_graphs,
    progress,
    requirements,
    topics,
    users,
);
