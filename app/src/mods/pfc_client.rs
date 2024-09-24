static SEARCH_URL: &str = "https://calorie.slism.jp/?search=検索&searchWord=";

pub fn get_search_results(search_word: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
  let url = format!("{}{}", SEARCH_URL, search_word);
  let body = reqwest::blocking::get(&url)?.text()?;
  let document = scraper::Html::parse_document(&body);
  let selector = scraper::Selector::parse("tr.searchItem > td.haba04").unwrap();
  let search_result_body = document.select(&selector);

  let mut results = Vec::new();
  for element in search_result_body {
      let name_selector = scraper::Selector::parse("a.searchName").unwrap();
      let name = element.select(&name_selector).next().unwrap().text().collect::<String>();
      let calorie_selector = scraper::Selector::parse("span.ccds_unit").unwrap();
      // ex. gram_text = 1膳(160g)
      let gram_text = element.select(&calorie_selector).next().unwrap().text().collect::<String>();

      let gram = gram_text.split("(").collect::<Vec<&str>>()[1].split("g").collect::<Vec<&str>>()[0].to_string();
      let unit = gram_text.split("(").collect::<Vec<&str>>()[0].to_string();


      results.push(vec![name, gram, unit]);
  }

  Ok(results)
}
