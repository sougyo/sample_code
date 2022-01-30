use actix_files as fs;
use actix_web::{get, App, Error, HttpRequest, HttpServer, HttpResponse, Responder};
use std::path::{Path, PathBuf};

#[get("/{filename:.+}")]
async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
	let dir_path  = Path::new("public_html");
	let base_path = PathBuf::from(req.match_info().query("filename"));
	Ok(fs::NamedFile::open(dir_path.join(base_path))?)
}

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| App::new().service(hello).service(index))
		.bind("0.0.0.0:8080")?
		.run()
		.await
}
