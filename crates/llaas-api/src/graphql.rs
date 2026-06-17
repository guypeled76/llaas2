use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};

pub type LlaasSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> LlaasSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn server_info(&self) -> ServerInfo {
        ServerInfo {
            name: "LLAAS".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    async fn languages(&self) -> Vec<Language> {
        vec![
            Language {
                name: "English".to_string(),
                code: "en".to_string(),
            },
            Language {
                name: "Spanish".to_string(),
                code: "es".to_string(),
            },
            Language {
                name: "French".to_string(),
                code: "fr".to_string(),
            },
            Language {
                name: "German".to_string(),
                code: "de".to_string(),
            },
        ]
    }
}

#[derive(SimpleObject)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(SimpleObject)]
pub struct Language {
    pub name: String,
    pub code: String,
}

#[cfg(test)]
mod tests {
    use super::schema;

    #[test]
    fn exports_expected_sdl() {
        let sdl = schema().sdl();

        assert!(sdl.contains("type QueryRoot"));
        assert!(sdl.contains("serverInfo"));
        assert!(sdl.contains("languages"));
    }
}
