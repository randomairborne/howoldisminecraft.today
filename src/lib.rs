#![feature(once_cell)]

mod parsed_manifest;
mod version_manifest;
pub use version_manifest::update_manifest;

use crate::parsed_manifest::LATEST_MANIFEST;
use chrono::{DateTime, Duration, Utc};
use gotham::helpers::http::response::{create_response, create_temporary_redirect};
use gotham::hyper::{Body, Response, StatusCode};
use gotham::prelude::*;
use gotham::router::{build_simple_router, Router};
use gotham::state::State;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct VersionRequestData {
    version: String,
}

pub fn idx(state: State) -> (State, Response<Body>) {
    let resp = create_temporary_redirect(&state, "/1.0");
    (state, resp)
}

pub fn version_age(mut state: State) -> (State, Response<Body>) {
    let st = std::time::Instant::now();

    let ver = std::mem::take(&mut VersionRequestData::borrow_mut_from(&mut state).version);

    let manifest = LATEST_MANIFEST
        .get()
        .expect("manifest not set, wait for initial update");
    let v = match manifest.versions.get(&ver) {
        Some(v) => *v,
        None => {
            let response = create_response(
                &state,
                StatusCode::NOT_FOUND,
                mime::TEXT_PLAIN,
                r#"<!DOCTYPE html>
<html>
  <head>
      <title>How old is minecraft?</title>
      <link rel="\#shortcut icon" type="image/png" href="https://dl.mcfix.org/howoldisminecraft.today/favicon.png">
  </head>
  <body>
  <h1 id="age" style="font-size: 32px; font-size: 3vw; height: 100%; width: 100%; display: flex; position: fixed; align-items: center; justify-content: center;">No version with that name exists!</h1><br>
  </body>
</html>"#,
            );
            return (state, response);
        }
    };

    let now: DateTime<Utc> = chrono::DateTime::from(std::time::SystemTime::now());
    let mut delta = now.naive_utc() - v;

    delta = delta
        - Duration::nanoseconds(
            delta.num_nanoseconds().expect("overflow should not happen")
                % (delta.num_days() * 86_400_000_000_000),
        );

    let age = chrono_humanize::HumanTime::from(delta).to_text_en(
        chrono_humanize::Accuracy::Precise,
        chrono_humanize::Tense::Present,
    );

    let mut ret = String::with_capacity(420 + ver.len() + age.len());
    write!(ret,
           r#"<!DOCTYPE html>
<html>
  <head>
      <title>How old is minecraft?</title>
      <link rel="\#shortcut icon" type="image/png" href="https://dl.mcfix.org/howoldisminecraft.today/favicon.png">
  </head>
  <body>
  <h1 id="age" style="font-size: 32px; font-size: 3vw; height: 100%; width: 100%; display: flex; position: fixed; align-items: center; justify-content: center;">Minecraft {} is {} old</h1><br>
  </body>
</html>"#, ver, age).expect("failed to write to buffer");

    let response = create_response(&state, StatusCode::OK, mime::TEXT_HTML_UTF_8, ret);

    let ret = (state, response);

    let et = std::time::Instant::now();
    let tt = et.duration_since(st).as_nanos();
    println!("took {}ns to respond to request", tt);

    ret
}

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(idx);
        route.get("/favicon.ico").to_file("assets/favicon.ico");
        route.get("/robots.txt").to_file("assets/robots.txt");
        route.get("/sitemap.xml").to_file("assets/sitemap.xml");
        route
            .get("/:version")
            .with_path_extractor::<VersionRequestData>()
            .to(version_age);
    })
}
