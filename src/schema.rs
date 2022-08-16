use cipherstash_client::{
    collection,
    indexer::{
        mapping::{Mapping, MappingWithMeta},
        CollectionSchema,
    },
    record::decoder::{DataType, RecordSchema, SchemaField},
};
use uuid::Uuid;

pub fn collection_schema() -> CollectionSchema {
    CollectionSchema {
        schema: RecordSchema {
            map: collection! {
                "title" => SchemaField::DataType(DataType::String),
                "runningTime" => SchemaField::DataType(DataType::Uint64)
            },
        },
        indexes: collection! {
            "exactTitle" => MappingWithMeta {
                mapping: Mapping::Exact { field: "title".into() },
                index_id: *Uuid::parse_str("65e381f5-18dc-41af-8a89-d0641b09accc").unwrap().as_bytes(),
                prf_key: [0;16],
                prp_key: [0;16]
            },
            "runningTime" => MappingWithMeta {
                mapping: Mapping::Range { field: "runningTime".into() },
                index_id: *Uuid::parse_str("f83f2cf4-4fc9-4147-9c39-7b99eafa74df").unwrap().as_bytes(),
                prf_key: [0;16],
                prp_key: [0;16]
            }
        },
    }
}
