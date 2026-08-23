use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

extern crate trpl; // required for mdbook test

fn main() {
    // TODO: we'll add this next!
}
