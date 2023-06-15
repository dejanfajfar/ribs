pub mod entities;

use entities::ArmorEntity;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

pub async fn get_all_armors() -> surrealdb::Result<Vec<ArmorEntity>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("ribs").await?;

    let armors: Vec<ArmorEntity> = db.select("armors").await?;

    return Ok(armors);
}

pub async fn add_armor(entity: &ArmorEntity) -> surrealdb::Result<ArmorEntity>{
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("ribs").await?;

    let new_armor = db
    .create(("armors", entity.armor_id.clone()))
    .content(entity)
    .await?;

    return Ok(new_armor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo(){
        let foo = get_all_armors();
    }
}