/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[derive(PostgresOrd)]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::pgx_sql::PgxSql;
use crate::to_sql::entity::ToSqlConfigEntity;
use crate::to_sql::ToSql;
use crate::{SqlGraphEntity, SqlGraphIdentifier};
use std::cmp::Ordering;

/// The output of a [`PostgresOrd`](crate::postgres_ord::PostgresOrd) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostgresOrdEntity {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub to_sql_config: ToSqlConfigEntity,
}

impl PostgresOrdEntity {
    pub(crate) fn cmp_fn_name(&self) -> String {
        format!("{}_cmp", self.name.to_lowercase())
    }

    pub(crate) fn lt_fn_name(&self) -> String {
        format!("{}_lt", self.name.to_lowercase())
    }

    pub(crate) fn le_fn_name(&self) -> String {
        format!("{}_le", self.name.to_lowercase())
    }

    pub(crate) fn eq_fn_name(&self) -> String {
        format!("{}_eq", self.name.to_lowercase())
    }

    pub(crate) fn gt_fn_name(&self) -> String {
        format!("{}_gt", self.name.to_lowercase())
    }

    pub(crate) fn ge_fn_name(&self) -> String {
        format!("{}_ge", self.name.to_lowercase())
    }
}

impl Ord for PostgresOrdEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file.cmp(other.file).then_with(|| self.file.cmp(other.file))
    }
}

impl PartialOrd for PostgresOrdEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<PostgresOrdEntity> for SqlGraphEntity {
    fn from(val: PostgresOrdEntity) -> Self {
        SqlGraphEntity::Ord(val)
    }
}

impl SqlGraphIdentifier for PostgresOrdEntity {
    fn dot_identifier(&self) -> String {
        format!("ord {}", self.full_path)
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

impl ToSql for PostgresOrdEntity {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, _context: &PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            CREATE OPERATOR FAMILY {name}_btree_ops USING btree;\n\
                            CREATE OPERATOR CLASS {name}_btree_ops DEFAULT FOR TYPE {name} USING btree FAMILY {name}_btree_ops AS\n\
                                  \tOPERATOR 1 <,\n\
                                  \tOPERATOR 2 <=,\n\
                                  \tOPERATOR 3 =,\n\
                                  \tOPERATOR 4 >=,\n\
                                  \tOPERATOR 5 >,\n\
                                  \tFUNCTION 1 {cmp_fn_name}({name}, {name});\
                            ",
                          name = self.name,
                          full_path = self.full_path,
                          file = self.file,
                          line = self.line,
                          cmp_fn_name = self.cmp_fn_name(),
        );
        tracing::trace!(%sql);
        Ok(sql)
    }
}
