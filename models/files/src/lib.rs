use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, Upload, ID};
use futures::lock::Mutex;
use slab::Slab;

pub type FilesSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, SimpleObject)]
pub struct FileInfo {
    id: ID,
    url: String,
}

pub type Storage = Mutex<Slab<FileInfo>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn uploads(&self, ctx: &Context<'_>) -> Vec<FileInfo> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.iter().map(|(_, file)| file).cloned().collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn single_upload(&self, ctx: &Context<'_>, file: Upload) -> FileInfo {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        println!("files count: {}", storage.len());
        let entry = storage.vacant_entry();
        let upload = file.value(ctx).unwrap();
        let info = FileInfo {
            id: entry.key().into(),
            url: upload.filename,
        };
        entry.insert(info.clone());
        info
    }

    async fn multiple_upload(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Vec<FileInfo> {
        let mut infos = Vec::new();
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        for file in files {
            let entry = storage.vacant_entry();
            let upload = file.value(ctx).unwrap();
            let info = FileInfo {
                id: entry.key().into(),
                url: upload.filename.clone(),
            };
            entry.insert(info.clone());
            infos.push(info)
        }
        infos
    }
}
