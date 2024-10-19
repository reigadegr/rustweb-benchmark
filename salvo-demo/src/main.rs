extern crate rbatis;
// extern crate rbdc;
mod config;
mod controller;
mod mapper;
mod pojo;
mod res;
mod services;

use mimalloc::MiMalloc;

use config::init::init_salvo_framework;
// use config::nacos::init_nacos_service;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
#[tokio::main]
async fn main() {
    // init_nacos_service().await;
    init_salvo_framework().await;
}
