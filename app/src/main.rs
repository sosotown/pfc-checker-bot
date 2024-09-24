mod mods::pfc_client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_word = "ごはん";
    let search_results = pfc_client::get_search_results(search_word)?;

    for result in search_results {
        println!("name: {}, gram: {}, unit: {}", result[0], result[1], result[2]);
    }

    Ok(())
}
