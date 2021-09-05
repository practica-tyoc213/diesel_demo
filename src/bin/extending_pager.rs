//use diesel::backend::{BindCollector, QueryBuilder};
use diesel::impl_query_id; // deprecated since 1.1.0
use diesel::pg::Pg;
use diesel::query_builder::{AsQuery, AstPass, Query, QueryFragment};
use diesel::sql_types::BigInt;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use diesel::{QueryId, Queryable};
use diesel_demo::establish_connection;
use diesel_demo::models::Post;
use diesel_demo::schema::posts;

// Implement `QueryFragment`
impl<T> QueryFragment<Pg> for PaginatedQuery<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") as internal LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.page)?;
        Ok(())
    }
}

impl_query_id!(PaginatedQuery<T>);

impl<T: Query> Query for PaginatedQuery<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for PaginatedQuery<T> {}

pub trait Paginate: AsQuery + Sized {
    fn paginate(self, page: i64) -> PaginatedQuery<Self::Query> {
        PaginatedQuery {
            query: self.as_query(),
            page,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl<T: AsQuery> Paginate for T {}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Queryable)]
pub struct PaginatedQuery<T> {
    query: T,
    page: i64,
    per_page: i64,
}

impl<T> PaginatedQuery<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        PaginatedQuery { per_page, ..self }
    }
}

fn main() {
    let paginated_query = posts::table.paginate(3).per_page(2);
    println!("count hay {:?}", paginated_query);
    let conn = establish_connection();
    let e: Vec<Post> = paginated_query.get_results(&conn).expect("not working");
    println!("execution {:?}", e);
}
