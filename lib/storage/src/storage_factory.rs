use std::sync::Arc;

use rdf_fusion_extensions::storage::QuadStorage;
use rdf_fusion_encoding::object_id::{ObjectIdEncoding, ObjectIdMapping};
use crate::memory::{MemObjectIdMapping, MemQuadStorage};
use crate::oxigraph_memory::MemoryQuadStorage;

#[derive(Debug)]
pub enum StorageBackend {
    DefaultMemory,
    OxigraphMemory,
}

impl StorageBackend {
    pub fn from_env() -> Self {
        match std::env::var("RDF_FUSION_STORAGE").as_deref() {
            Ok("oxigraph") => Self::OxigraphMemory,
            Ok("default") | Ok("memory") | _ => Self::DefaultMemory,
        }
    }
}

pub fn create_storage_from_env() -> Arc<dyn QuadStorage> {
    let backend = StorageBackend::from_env();
    println!("Using storage backend: {:?}", backend);

    match backend {
        StorageBackend::DefaultMemory => {
            let object_id_mapping = Arc::new(MemObjectIdMapping::new());
            let encoding = Arc::new(ObjectIdEncoding::new(
                Arc::clone(&object_id_mapping) as Arc<dyn ObjectIdMapping>
            ));
            Arc::new(MemQuadStorage::new(object_id_mapping, encoding, 8192))
        }
        StorageBackend::OxigraphMemory => {
            Arc::new(MemoryQuadStorage::new())
        }
    }
}