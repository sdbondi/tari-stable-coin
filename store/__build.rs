// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use sqlx::ConnectOptions;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use tokio::runtime;

fn main() {
    println!("cargo:warning=HERE");
    if env::var("__SKIP_SQLX_BUILD").is_ok() {
        return;
    }

    let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:warning={}", cargo_dir.display());

    let _ = fs::remove_file(cargo_dir.join("data/__tmp_store.db"));
    fs::create_dir_all(cargo_dir.join("data")).unwrap();
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async {
        let mut conn = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(cargo_dir.join("data/__tmp_store.db"))
            .create_if_missing(true)
            .connect()
            .await
            .unwrap();

        sqlx::migrate!("./migrations").run(&mut conn).await.unwrap();
    });

    let out = Command::new("cargo")
        .arg("sqlx")
        .arg("prepare")
        .arg("--database-url")
        .arg(format!(
            "sqlite://{}",
            cargo_dir.join("data/__tmp_store.db").display()
        ))
        .env("__SKIP_SQLX_BUILD", "1")
        .env(
            "DATABASE_URL",
            format!(
                "sqlite://{}",
                cargo_dir.join("data/__tmp_store.db").display()
            ),
        )
        .output()
        .expect("Failed to run sqlx prepare");

    println!("cargo:warning={}", String::from_utf8_lossy(&out.stdout));
    println!("cargo:warning={}", String::from_utf8_lossy(&out.stderr));
    env::set_var(
        "DATABASE_URL",
        format!(
            "sqlite://{}",
            cargo_dir.join("data/__tmp_store.db").display()
        ),
    );
}
