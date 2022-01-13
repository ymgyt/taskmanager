use async_graphql::{Object,Context,SimpleObject,ComplexObject,
    connection::{Connection,Edge},
};

use sea_orm::DatabaseConnection;
use chrono::{DateTime, offset::{FixedOffset}, Utc};

use crate::db::orm::tasks::{self, Entity as TaskEntity, Model as TaskModel};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn echo(&self, content: String) -> String {
        content.to_uppercase()
    }

    async fn add(&self, x: i64, y: i64) -> i64 {
        x + y
    }

    async fn simple_obj<'ctx>(&self, ctx: &Context<'ctx>) -> MySimpleObject {
        let _db_conn = ctx.data_unchecked::<sea_orm::DatabaseConnection>();
        MySimpleObject { a: "A".to_owned(), b: 100 }
    }

    async fn tasks<'ctx>(
        &self, ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<usize>,
        last: Option<usize>,
    ) -> Connection<String,Task> {
        query_tasks(ctx, after,before, first, last).await
    }

    async fn task<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<Task> {
        let db = ctx.data_unchecked::<DatabaseConnection>();

        use sea_orm::EntityTrait;
        TaskEntity::find_by_id(id).one(db).await.unwrap().map(Task)
    }
}

struct Task(TaskModel);

#[Object]
impl Task {
    async fn id(&self) -> String { self.0.id.clone() }
    async fn title(&self) -> String { self.0.title.clone() }
    async fn dependent_tasks<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<usize>,
        last: Option<usize>,
    ) -> Connection<String,Task> {
        query_tasks(ctx, after, before, first, last).await
    }
}

#[allow(clippy::needless_lifetimes)]
async fn query_tasks<'ctx>(
    ctx: &Context<'ctx>,
    after: Option<String>,
    _before: Option<String>,
    first: Option<usize>,
    _last: Option<usize>,
) -> Connection<String, Task> {
    let db = ctx.data_unchecked::<DatabaseConnection>();

    let after = if let Some(after) = after {
        let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&after).unwrap();
        dt
    } else {
        Utc::now().with_timezone(&FixedOffset::east(0))
    };

    let page_size = first.unwrap_or(10);
    // orderは現状、created_at固定
    // cursorには必ずulidを含むようにして、第2sort keyとして利用する
    use sea_orm::{EntityTrait,QueryOrder,QuerySelect,QueryFilter, ColumnTrait};
    let ts = TaskEntity::find()
        .order_by_desc(tasks::Column::CreatedAt)
        .find_with_related(TaskEntity)
        .limit(page_size as u64 + 1)
        .filter(tasks::Column::CreatedAt.gte(after))
        .all(db).await.unwrap();
    let ts = dbg!(ts);

    // let mut conn = Connection::new(false,ts.len() > page_size);
    // conn.append(ts.into_iter().map(|t| Edge::new(t.created_at.to_string(), Task(t))));
    //
    // conn
    todo!()
}


#[derive(SimpleObject)]
#[graphql(complex)]
struct MySimpleObject {
    a: String,
    b: u64,
}

#[ComplexObject]
impl MySimpleObject {
    async fn c(&self) -> String {
        format!("{} {}", self.a, self.b)
    }
}
