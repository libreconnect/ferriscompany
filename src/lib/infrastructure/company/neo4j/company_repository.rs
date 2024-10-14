use std::sync::Arc;

use neo4rs::{query, Query};

use crate::{
    domain::company::{
        models::{Company, CompanyError},
        ports::CompanyRepository,
    },
    infrastructure::db::neo4j::Neo4j,
};

#[derive(Clone)]
pub struct Neo4jCompanyRepository {
    neo4j: Arc<Neo4j>,
}

impl Neo4jCompanyRepository {
    pub fn new(neo4j: Arc<Neo4j>) -> Neo4jCompanyRepository {
        Neo4jCompanyRepository { neo4j }
    }
}

impl CompanyRepository for Neo4jCompanyRepository {
    async fn delete(&self, id: uuid::Uuid) -> Result<(), CompanyError> {
        let query = query("MATCH (c:Company {id: $id}) DELETE c").param("id", id.to_string());

        self.neo4j
            .get_graph()
            .execute(query)
            .await
            .map_err(|_| CompanyError::DeleteError)
            .map(|_| ())
    }

    async fn save(&self, company: &Company) -> Result<(), CompanyError> {
        let query = query(
            "
            CREATE (c:Company {id: $id, name: $name})
            RETURN c
        ",
        )
        .param("id", company.id.to_string())
        .param("name", company.name.as_str());

        self.neo4j
            .get_graph()
            .execute(query)
            .await
            .map_err(|e| CompanyError::CreateError(e.to_string()))
            .map(|_| ())
    }

    async fn find_all(&self) -> Result<Vec<Company>, CompanyError> {
        todo!()
    }

    async fn find_by_id(&self, _id: uuid::Uuid) -> Result<Option<Company>, CompanyError> {
        todo!()
    }
}
