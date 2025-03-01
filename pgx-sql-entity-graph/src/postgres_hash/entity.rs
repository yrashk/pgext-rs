/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[derive(PostgresHash)]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::pgx_sql::PgxSql;
use crate::to_sql::entity::ToSqlConfigEntity;
use crate::to_sql::ToSql;
use crate::{SqlGraphEntity, SqlGraphIdentifier};
use std::cmp::Ordering;

/// The output of a [`PostgresHash`](crate::postgres_hash::PostgresHash) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostgresHashEntity {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub to_sql_config: ToSqlConfigEntity,
}

impl PostgresHashEntity {
    pub(crate) fn fn_name(&self) -> String {
        format!("{}_hash", self.name.to_lowercase())
    }
}

impl Ord for PostgresHashEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file.cmp(other.file).then_with(|| self.file.cmp(other.file))
    }
}

impl PartialOrd for PostgresHashEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<PostgresHashEntity> for SqlGraphEntity {
    fn from(val: PostgresHashEntity) -> Self {
        SqlGraphEntity::Hash(val)
    }
}

impl SqlGraphIdentifier for PostgresHashEntity {
    fn dot_identifier(&self) -> String {
        format!("hash {}", self.full_path)
    }
    fn rust_identifier(&self) -> String {
        self.full_path.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for PostgresHashEntity {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, _context: &PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            CREATE OPERATOR FAMILY {name}_hash_ops USING hash;\n\
                            CREATE OPERATOR CLASS {name}_hash_ops DEFAULT FOR TYPE {name} USING hash FAMILY {name}_hash_ops AS\n\
                                \tOPERATOR    1   =  ({name}, {name}),\n\
                                \tFUNCTION    1   {fn_name}({name});\
                            ",
                          name = self.name,
                          full_path = self.full_path,
                          file = self.file,
                          line = self.line,
                          fn_name = self.fn_name(),
        );
        tracing::trace!(%sql);
        Ok(sql)
    }
}
