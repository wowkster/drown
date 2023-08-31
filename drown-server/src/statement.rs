#[derive(Debug)]
pub enum Statement {
    Select(SelectStatement),
}

#[derive(Debug)]
pub struct SelectStatement {
    columns: ResultColumns,
    from_clause: FromClause,
    where_clause: Option<WhereClause>,
    group_by_clause: Option<GroupByClause>,
    having_clause: Option<HavingClause>,
    order_by_clause: Option<OrderByClause>,
    offset: Option<OffsetClause>,
    limit: Option<LimitClause>,
}

/* SELECT */

#[derive(Debug)]
pub enum ResultColumns {
    /// SELECT *
    All,
    /// SELECT a, b, c
    /// or
    /// SELECT a AS b, c AS d
    Specific(Vec<AliasedColumnName>),
}

#[derive(Debug)]
pub struct AliasedColumnName {
    column_name: ColumnName,
    alias: Option<String>,
}

#[derive(Debug)]
pub enum ColumnName {
    /// a
    Direct { name: String },
    /// a.b
    Qualified {
        table_name: String,
        column_name: String,
    }
}

/* FROM */

#[derive(Debug)]
pub enum FromClause {
    /// FROM a
    /// FROM a AS b
    /// FROM (SELECT ...)
    /// FROM (SELECT ...) AS a
    Direct {
        table: AliasedFromTable,
    },
    /// FROM a JOIN b ON ...
    Join {
        left: Box<AliasedFromTable>,
        right: Box<AliasedFromTable>,
        join_type: JoinType,
        join_condition: JoinCondition,
    },
}

/// (SELECT ...) [AS a]
/// a [AS b]
#[derive(Debug)]
pub struct AliasedFromTable {
    table: FromTable,
    alias: Option<String>,
}

#[derive(Debug)]
pub enum FromTable {
    /// (SELECT ...)
    DerivedTable { query: Box<SelectStatement> },
    /// a
    TableName { name: String },
}

#[derive(Debug)]
pub enum JoinType {
    Inner,
    LeftOuter,
    RightOuter,
    FullOuter,
}

#[derive(Debug)]
pub enum JoinCondition {
    On(Expr),
    Using(Vec<String>),
}

#[derive(Debug)]
pub enum Expr {}

/* WHERE */

#[derive(Debug)]
pub struct WhereClause;

/* GROUP BY */

#[derive(Debug)]
pub struct GroupByClause {
    groupings: Vec<ColumnName>,
}

/* HAVING */

#[derive(Debug)]
pub struct HavingClause;

/* ORDER BY */

#[derive(Debug)]
pub struct OrderByClause {
    orderings: Vec<OrderByItem>,
}

#[derive(Debug)]
pub struct OrderByItem {
    column_name: ColumnName,
    ordering: Option<Ordering>,
}

#[derive(Debug)]
pub enum Ordering {
    Ascending,
    Descending,
}

/* OFFSET */

#[derive(Debug)]
pub struct OffsetClause {
    offset: u64,
}

/* LIMIT */

#[derive(Debug)]
pub struct LimitClause {
    limit: u64,
}