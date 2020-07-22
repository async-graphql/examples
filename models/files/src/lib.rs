use async_graphql::{Context, EmptySubscription, Schema, Upload, ID};
use futures::lock::Mutex;
use slab::Slab;

pub type FilesSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[async_graphql::SimpleObject]
#[derive(Clone)]
pub struct FileInfo {
    id: ID,
    filename: String,
    mimetype: Option<String>,
}

pub type Storage = Mutex<Slab<FileInfo>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn uploads(&self, ctx: &Context<'_>) -> Vec<FileInfo> {
        let storage = ctx.data::<Storage>().lock().await;
        storage.iter().map(|(_, file)| file).cloned().collect()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn single_upload(&self, ctx: &Context<'_>, file: Upload) -> FileInfo {
        let mut storage = ctx.data::<Storage>().lock().await;
        println!("files count: {}", storage.len());
        let entry = storage.vacant_entry();
        let info = FileInfo {
            id: entry.key().into(),
            filename: file.filename().to_string(),
            mimetype: file.content_type().map(ToString::to_string),
        };
        entry.insert(info.clone());
        info
    }

    async fn multiple_upload(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Vec<FileInfo> {
        let mut infos = Vec::new();
        let mut storage = ctx.data::<Storage>().lock().await;
        for file in files {
            let entry = storage.vacant_entry();
            let info = FileInfo {
                id: entry.key().into(),
                filename: file.filename().to_string(),
                mimetype: file.content_type().map(ToString::to_string),
            };
            entry.insert(info.clone());
            infos.push(info)
        }
        infos
    }
}
