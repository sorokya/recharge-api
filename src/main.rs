use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tokio::{fs::File, io::AsyncReadExt};

mod decode_int;
mod dump_classes;
mod dump_items;
mod dump_npcs;
mod dump_spells;
mod ecf_record;
mod eif_record;
mod enf_record;
mod esf_record;
mod read;
mod write_json_file;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dump_items::dump_items().await?;
    dump_npcs::dump_npcs().await?;
    dump_classes::dump_classes().await?;
    dump_spells::dump_spells().await?;

    let config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&config))
            .service(npc)
            .service(item)
            .service(class)
            .service(spell)
    })
    .bind(("127.0.0.1", 27631))?
    .run()
    .await
}

#[get("/npc/{id}")]
async fn npc(path: web::Path<u32>) -> impl Responder {
    let mut file = match File::open(format!("dump/npcs/{}.json", path.into_inner())).await {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("NPC not found"),
    };

    let mut buf = Vec::with_capacity(file.metadata().await.unwrap().len() as usize);

    match file.read_to_end(&mut buf).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read NPC file"),
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(buf)
}

#[get("/item/{id}")]
async fn item(path: web::Path<u32>) -> impl Responder {
    let mut file = match File::open(format!("dump/items/{}.json", path.into_inner())).await {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("Item not found"),
    };

    let mut buf = Vec::with_capacity(file.metadata().await.unwrap().len() as usize);

    match file.read_to_end(&mut buf).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read Item file"),
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(buf)
}

#[get("/class/{id}")]
async fn class(path: web::Path<u32>) -> impl Responder {
    let mut file = match File::open(format!("dump/classes/{}.json", path.into_inner())).await {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("Class not found"),
    };

    let mut buf = Vec::with_capacity(file.metadata().await.unwrap().len() as usize);

    match file.read_to_end(&mut buf).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read Class file"),
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(buf)
}

#[get("/spell/{id}")]
async fn spell(path: web::Path<u32>) -> impl Responder {
    let mut file = match File::open(format!("dump/spells/{}.json", path.into_inner())).await {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("Spell not found"),
    };

    let mut buf = Vec::with_capacity(file.metadata().await.unwrap().len() as usize);

    match file.read_to_end(&mut buf).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read Spell file"),
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(buf)
}
