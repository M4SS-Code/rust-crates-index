use crates_index::SparseIndex;

///
/// command to run:<br>
/// cargo run --example sparse_http_ureq -F sparse
///

const CRATE_TO_FETCH: &str = "inferno";

fn main() {
    let mut index = SparseIndex::new_cargo_default().unwrap();

    print_crate(&mut index);
    update(&mut index);
    print_crate(&mut index);
}

fn print_crate(index: &mut SparseIndex) {
    match index.crate_from_cache(CRATE_TO_FETCH) {
        Ok(krate) => {
            println!("{:?}", krate.highest_normal_version().unwrap().version());
        }
        Err(_err) => {
            println!("could not find crate {}", CRATE_TO_FETCH)
        }
    }
}

fn update(index: &mut SparseIndex) {
    // ureq doesn't support HTTP/2, so we have to set the version to HTTP/1.1
    let request = index
        .make_cache_request(CRATE_TO_FETCH)
        .unwrap()
        .version(ureq::http::Version::HTTP_11)
        .body(())
        .unwrap();

    let response = ureq::run(request).unwrap();

    let mut builder = http::Response::builder()
        .status(response.status())
        .version(response.version());
    builder
        .headers_mut()
        .unwrap()
        .extend(response.headers().iter().map(|(k, v)| (k.clone(), v.clone())));
    let response = builder.body(response.into_body().read_to_vec().unwrap()).unwrap();

    index.parse_cache_response(CRATE_TO_FETCH, response, true).unwrap();
}
