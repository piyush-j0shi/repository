use trpl::Html;

// async fn block is not like a normal function because it would not run automatically it will return a future so it will not run until you use `.await` with it
// and once you use `.await` it will return the result.

// so basically async fn page_title(url : &str) -> Option<String>
// It means
// fn page_title(url : &str) -> impl Future<Output = Option<String>> {
//      async {
//          function body here
//      }
// }

async fn page_title(url: &str) -> Option<String> {
    let reponse_text = trpl::get(url).await.text().await;
    Html::parse(&reponse_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} is {title}"),
            None => println!("{url} had no title"),
        }
    })
}
