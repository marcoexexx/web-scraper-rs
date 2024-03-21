#![allow(unused)]

use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::Browser;
use scraper::{Html, Selector};
use std::{error, fs, path::Path, thread, time::Duration};

const URL: &str = "https://scrapeme.live/shop/";

#[derive(Debug)]
struct PokemonProduct {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

fn with_scraper() -> Result<(), Box<dyn error::Error>> {
    let res = reqwest::blocking::get(URL)?;
    let html_content = res.text()?;

    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    let document = Html::parse_document(&html_content);
    let product_selector = Selector::parse("li.product")?;

    let html_products = document.select(&product_selector);

    for html_product in html_products {
        let url = html_product
            .select(&Selector::parse("a")?)
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = html_product
            .select(&Selector::parse("img")?)
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let name = html_product
            .select(&Selector::parse("h2")?)
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = html_product
            .select(&Selector::parse(".price")?)
            .next()
            .map(|price| price.text().collect::<String>());

        let pokemon = PokemonProduct {
            url,
            image,
            name,
            price,
        };
        pokemon_products.push(pokemon);
    }

    println!("{:#?}", pokemon_products);

    Ok(())
}

fn with_headless() -> Result<(), Box<dyn error::Error>> {
    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.navigate_to(URL)?;
    tab.wait_until_navigated()?;

    thread::sleep(Duration::from_secs(5));

    fs::write(Path::new("content.html"), tab.get_content()?)?;

    let ss = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, Some(27), None, true)?;

    fs::write(Path::new("ss.png"), ss)?;

    Ok(())
}

fn main() {
    // with_scraper().unwrap();
    with_headless().unwrap();
}
