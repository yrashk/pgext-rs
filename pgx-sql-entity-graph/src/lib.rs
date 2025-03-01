/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

Rust to SQL mapping support.

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
pub use aggregate::entity::{AggregateTypeEntity, PgAggregateEntity};
pub use aggregate::{
    AggregateType, AggregateTypeList, FinalizeModify, ParallelOption, PgAggregate,
};
pub use control_file::ControlFile;
pub use enrich::CodeEnrichment;
pub use extension_sql::entity::{ExtensionSqlEntity, SqlDeclaredEntity};
pub use extension_sql::{ExtensionSql, ExtensionSqlFile, SqlDeclared};
pub use extern_args::{parse_extern_attributes, ExternArgs};
pub use mapping::{RustSourceOnlySqlMapping, RustSqlMapping};
pub use pg_extern::entity::{
    PgExternArgumentEntity, PgExternEntity, PgExternReturnEntity, PgExternReturnEntityIteratedItem,
    PgOperatorEntity,
};
pub use pg_extern::{NameMacro, PgExtern, PgExternArgument, PgOperator};
pub use pg_trigger::attribute::PgTriggerAttribute;
pub use pg_trigger::entity::PgTriggerEntity;
pub use pg_trigger::PgTrigger;
pub use pgx_sql::{PgxSql, RustToSqlMapping};
pub use positioning_ref::PositioningRef;
pub use postgres_enum::entity::PostgresEnumEntity;
pub use postgres_enum::PostgresEnum;
pub use postgres_hash::entity::PostgresHashEntity;
pub use postgres_hash::PostgresHash;
pub use postgres_ord::entity::PostgresOrdEntity;
pub use postgres_ord::PostgresOrd;
pub use postgres_type::entity::PostgresTypeEntity;
pub use postgres_type::PostgresType;
pub use schema::entity::SchemaEntity;
pub use schema::Schema;
pub use to_sql::entity::ToSqlConfigEntity;
pub use to_sql::{ToSql, ToSqlConfig};
pub use used_type::{UsedType, UsedTypeEntity};

pub(crate) mod aggregate;
pub(crate) mod control_file;
pub(crate) mod enrich;
pub(crate) mod extension_sql;
pub(crate) mod extern_args;
pub mod lifetimes;
pub(crate) mod mapping;
pub mod metadata;
pub(crate) mod pg_extern;
pub(crate) mod pg_trigger;
pub(crate) mod pgx_attribute;
pub(crate) mod pgx_sql;
pub mod positioning_ref;
pub(crate) mod postgres_enum;
pub(crate) mod postgres_hash;
pub(crate) mod postgres_ord;
pub(crate) mod postgres_type;
pub(crate) mod schema;
pub(crate) mod to_sql;
pub(crate) mod used_type;

/// Able to produce a GraphViz DOT format identifier.
pub trait SqlGraphIdentifier {
    /// A dot style identifier for the entity.
    ///
    /// Typically this is a 'archetype' prefix (eg `fn` or `type`) then result of
    /// [`std::module_path`], [`core::any::type_name`], or some combination of [`std::file`] and
    /// [`std::line`].
    fn dot_identifier(&self) -> String;

    /// A Rust identifier for the entity.
    ///
    /// Typically this is the result of [`std::module_path`], [`core::any::type_name`],
    /// or some combination of [`std::file`] and [`std::line`].
    fn rust_identifier(&self) -> String;

    fn file(&self) -> Option<&'static str>;

    fn line(&self) -> Option<u32>;
}

/// An entity corresponding to some SQL required by the extension.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SqlGraphEntity {
    ExtensionRoot(ControlFile),
    Schema(SchemaEntity),
    CustomSql(ExtensionSqlEntity),
    Function(PgExternEntity),
    Type(PostgresTypeEntity),
    BuiltinType(String),
    Enum(PostgresEnumEntity),
    Ord(PostgresOrdEntity),
    Hash(PostgresHashEntity),
    Aggregate(PgAggregateEntity),
    Trigger(PgTriggerEntity),
}

impl SqlGraphEntity {
    pub fn sql_anchor_comment(&self) -> String {
        let maybe_file_and_line = if let (Some(file), Some(line)) = (self.file(), self.line()) {
            format!("-- {file}:{line}\n", file = file, line = line)
        } else {
            String::default()
        };
        format!(
            "\
            {maybe_file_and_line}\
            -- {rust_identifier}\
        ",
            maybe_file_and_line = maybe_file_and_line,
            rust_identifier = self.rust_identifier(),
        )
    }
}

impl SqlGraphIdentifier for SqlGraphEntity {
    fn dot_identifier(&self) -> String {
        match self {
            SqlGraphEntity::Schema(item) => item.dot_identifier(),
            SqlGraphEntity::CustomSql(item) => item.dot_identifier(),
            SqlGraphEntity::Function(item) => item.dot_identifier(),
            SqlGraphEntity::Type(item) => item.dot_identifier(),
            SqlGraphEntity::BuiltinType(item) => format!("preexisting type {}", item),
            SqlGraphEntity::Enum(item) => item.dot_identifier(),
            SqlGraphEntity::Ord(item) => item.dot_identifier(),
            SqlGraphEntity::Hash(item) => item.dot_identifier(),
            SqlGraphEntity::Aggregate(item) => item.dot_identifier(),
            SqlGraphEntity::Trigger(item) => item.dot_identifier(),
            SqlGraphEntity::ExtensionRoot(item) => item.dot_identifier(),
        }
    }

    fn rust_identifier(&self) -> String {
        match self {
            SqlGraphEntity::Schema(item) => item.rust_identifier(),
            SqlGraphEntity::CustomSql(item) => item.rust_identifier(),
            SqlGraphEntity::Function(item) => item.rust_identifier(),
            SqlGraphEntity::Type(item) => item.rust_identifier(),
            SqlGraphEntity::BuiltinType(item) => item.to_string(),
            SqlGraphEntity::Enum(item) => item.rust_identifier(),
            SqlGraphEntity::Ord(item) => item.rust_identifier(),
            SqlGraphEntity::Hash(item) => item.rust_identifier(),
            SqlGraphEntity::Aggregate(item) => item.rust_identifier(),
            SqlGraphEntity::Trigger(item) => item.rust_identifier(),
            SqlGraphEntity::ExtensionRoot(item) => item.rust_identifier(),
        }
    }

    fn file(&self) -> Option<&'static str> {
        match self {
            SqlGraphEntity::Schema(item) => item.file(),
            SqlGraphEntity::CustomSql(item) => item.file(),
            SqlGraphEntity::Function(item) => item.file(),
            SqlGraphEntity::Type(item) => item.file(),
            SqlGraphEntity::BuiltinType(_item) => None,
            SqlGraphEntity::Enum(item) => item.file(),
            SqlGraphEntity::Ord(item) => item.file(),
            SqlGraphEntity::Hash(item) => item.file(),
            SqlGraphEntity::Aggregate(item) => item.file(),
            SqlGraphEntity::Trigger(item) => item.file(),
            SqlGraphEntity::ExtensionRoot(item) => item.file(),
        }
    }

    fn line(&self) -> Option<u32> {
        match self {
            SqlGraphEntity::Schema(item) => item.line(),
            SqlGraphEntity::CustomSql(item) => item.line(),
            SqlGraphEntity::Function(item) => item.line(),
            SqlGraphEntity::Type(item) => item.line(),
            SqlGraphEntity::BuiltinType(_item) => None,
            SqlGraphEntity::Enum(item) => item.line(),
            SqlGraphEntity::Ord(item) => item.line(),
            SqlGraphEntity::Hash(item) => item.line(),
            SqlGraphEntity::Aggregate(item) => item.line(),
            SqlGraphEntity::Trigger(item) => item.line(),
            SqlGraphEntity::ExtensionRoot(item) => item.line(),
        }
    }
}

impl ToSql for SqlGraphEntity {
    #[tracing::instrument(level = "debug", skip(self, context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, context: &PgxSql) -> eyre::Result<String> {
        match self {
            SqlGraphEntity::Schema(item) => {
                if item.name != "public" && item.name != "pg_catalog" {
                    item.to_sql(context)
                } else {
                    Ok(String::default())
                }
            }
            SqlGraphEntity::CustomSql(item) => item.to_sql(context),
            SqlGraphEntity::Function(item) => {
                if let Some(result) = item.to_sql_config.to_sql(self, context) {
                    return result;
                }
                if context.graph.neighbors_undirected(context.externs.get(item).unwrap().clone()).any(|neighbor| {
                    let neighbor_item = &context.graph[neighbor];
                    match neighbor_item {
                        SqlGraphEntity::Type(PostgresTypeEntity { in_fn, in_fn_module_path, out_fn, out_fn_module_path, .. }) => {
                            let is_in_fn = item.full_path.starts_with(in_fn_module_path) && item.full_path.ends_with(in_fn);
                            if is_in_fn {
                                tracing::trace!(r#type = %neighbor_item.dot_identifier(), "Skipping, is an in_fn.");
                            }
                            let is_out_fn = item.full_path.starts_with(out_fn_module_path) && item.full_path.ends_with(out_fn);
                            if is_out_fn {
                                tracing::trace!(r#type = %neighbor_item.dot_identifier(), "Skipping, is an out_fn.");
                            }
                            is_in_fn || is_out_fn
                        },
                        _ => false,
                    }
                }) {
                    Ok(String::default())
                } else {
                    item.to_sql(context)
                }
            }
            SqlGraphEntity::Type(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::BuiltinType(_) => Ok(String::default()),
            SqlGraphEntity::Enum(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Ord(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Hash(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Aggregate(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Trigger(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::ExtensionRoot(item) => item.to_sql(context),
        }
    }
}

/// Validate that a given ident is acceptable to PostgreSQL
///
/// PostgreSQL places some restrictions on identifiers for things like functions.
///
/// Namely:
///
/// * It must be less than 64 characters
///
// This list is incomplete, you could expand it!
pub fn ident_is_acceptable_to_postgres(ident: &syn::Ident) -> Result<(), syn::Error> {
    // Roughly `pgx::pg_sys::NAMEDATALEN`
    //
    // Technically it **should** be that exactly, however this is `pgx-utils` and a this data is used at macro time.
    const POSTGRES_IDENTIFIER_MAX_LEN: usize = 64;

    let ident_string = ident.to_string();
    if ident_string.len() >= POSTGRES_IDENTIFIER_MAX_LEN {
        return Err(syn::Error::new(
            ident.span(),
            &format!(
                "Identifier `{}` was {} characters long, PostgreSQL will truncate identifiers with less than {POSTGRES_IDENTIFIER_MAX_LEN} characters, opt for an identifier which Postgres won't truncate",
                ident,
                ident_string.len(),
            )
        ));
    }

    Ok(())
}
