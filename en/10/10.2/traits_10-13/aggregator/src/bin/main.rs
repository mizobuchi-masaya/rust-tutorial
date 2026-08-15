use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        contents: String::from("contentis"),
    };
    let summary = article.summarize();

    println!("article.headline: {:?}", article.headline);
    println!("article.location: {:?}", article.location);
    println!("article.author: {:?}", article.author);
    println!("article.contents: {:?}", article.contents);
    println!("article.summary: {:?}", summary);

    let socialpost = SocialPost {
        username: String::from("username"),
        content: String::from("content"),
        reply: true,
        repost: true,
    };
    let summary = socialpost.summarize();

    println!("socialpost.username: {:?}", socialpost.username);
    println!("socialpost.content: {:?}", socialpost.content);
    println!("socialpost.reply: {:?}", socialpost.reply);
    println!("socialpost.repost: {:?}", socialpost.repost);
    println!("socialpoat.summary: {:?}", summary);
}
