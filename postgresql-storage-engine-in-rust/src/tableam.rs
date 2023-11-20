use pgrx::pg_sys::*;
use pgrx::prelude::*;
use pgrx::{rust_regtypein, PostgresType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// use pgrx::pgrx_sql_entity_graph::metadata::{Returns, ReturnsError};
// use pgrx::pgrx_sql_entity_graph::metadata::{ArgumentError, SqlMapping, SqlTranslatable};

#[derive(Debug, Copy, Clone, PostgresType)]
pub struct InMemoryTableAm {
    pub methods: TableAmRoutine,
}

impl InMemoryTableAm {
    pub const fn new() -> Self {
        InMemoryTableAm {
            methods: TableAmRoutine {
                type_: NodeTag::T_TableAmRoutine,
                slot_callbacks: None,
                scan_begin: None,
                scan_end: None,
                scan_rescan: None,
                scan_getnextslot: None,
                parallelscan_estimate: None,
                parallelscan_initialize: None,
                parallelscan_reinitialize: None,
                index_fetch_begin: None,
                index_fetch_reset: None,
                index_fetch_end: None,
                index_fetch_tuple: None,
                tuple_fetch_row_version: None,
                tuple_tid_valid: None,
                tuple_get_latest_tid: None,
                tuple_satisfies_snapshot: None,
                compute_xid_horizon_for_tuples: None,
                tuple_insert: None,
                tuple_insert_speculative: None,
                tuple_complete_speculative: None,
                multi_insert: None,
                tuple_delete: None,
                tuple_update: None,
                tuple_lock: None,
                finish_bulk_insert: None,
                relation_set_new_filenode: None,
                relation_nontransactional_truncate: None,
                relation_copy_data: None,
                relation_copy_for_cluster: None,
                relation_vacuum: None,
                scan_analyze_next_block: None,
                scan_analyze_next_tuple: None,
                index_build_range_scan: None,
                index_validate_scan: None,
                relation_size: None,
                relation_needs_toast_table: None,
                relation_toast_am: None,
                relation_fetch_toast_slice: None,
                relation_estimate_size: None,
                scan_bitmap_next_block: None,
                scan_bitmap_next_tuple: None,
                scan_sample_next_block: None,
                scan_sample_next_tuple: None,
            },
        }
    }
}

// unsafe impl SqlTranslatable for TableAmRoutineWrapper {
//     fn argument_sql() -> std::result::Result<SqlMapping, ArgumentError> {
//         // Ok(SqlMapping::Skip)
//         Ok(SqlMapping::As("pg_tam_rust".into()))
//     }
//     fn return_sql() -> std::result::Result<Returns, ReturnsError> {
//         // Ok(Returns::One(SqlMapping::Skip))
//         Ok(Returns::One(SqlMapping::As("pg_tam_rust".into())))
//     }
// }

impl Serialize for InMemoryTableAm {
    fn serialize<S>(&self, _serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(serde::ser::Error::custom("Not implemented"))
    }
}

impl<'de> Deserialize<'de> for InMemoryTableAm {
    fn deserialize<D>(_deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(serde::de::Error::custom("Not implemented"))
    }
}

// impl FromDatum for &InMemoryTableAm {
//     unsafe fn from_polymorphic_datum(datum: Datum, is_null: bool, _: Oid) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         if is_null {
//             None
//         } else {
//             Some(&InMemoryTableAm {
//                 methods: datum.value() as TableAmRoutine,
//             })
//         }
//     }
// }

impl IntoDatum for &InMemoryTableAm {
    fn into_datum(self) -> Option<Datum> {
        Some(Datum::from(&self.methods as *const _))
    }

    fn type_oid() -> Oid {
        rust_regtypein::<Self>()
    }
}

impl PartialEq for InMemoryTableAm {
    fn eq(&self, other: &Self) -> bool {
        self.methods.type_ == other.methods.type_
    }
}
