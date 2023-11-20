mod tableam;

use pgrx::pg_sys::*;
use pgrx::prelude::*;
use tableam::*;

pgrx::pg_module_magic!();

static INMEM_TABLEAM_METHODS: InMemoryTableAm = InMemoryTableAm::new();

extension_sql!(
    "CREATE ACCESS METHOD inmem TYPE TABLE HANDLER inmem_tableam_handler;"
    name = "tableam"
);

#[pg_extern(sql = r#"
CREATE OR REPLACE FUNCTION inmem_tableam_handler(internal) RETURNS table_am_handler 
AS '@MODULE_PATHNAME@', '@FUNCTION_NAME@'
LANGUAGE C /* Rust */
STRICT;
"#)]
fn inmem_tableam_handler() -> &'static InMemoryTableAm {
    log!(
        "In-memory Table Access Method routine: {:?}",
        &INMEM_TABLEAM_METHODS
    );

    &INMEM_TABLEAM_METHODS
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_inmem_tableam_handler() {
        assert_eq!(
            crate::InMemoryTableAm::new(),
            *crate::inmem_tableam_handler()
        );
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
