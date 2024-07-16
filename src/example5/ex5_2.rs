trait SummaryTitle {
    fn summarize_title(&self) -> String;
    fn summarize(&self) -> String {
        format!("summarize_title is : {} ", self.summarize_title())
    }
}

struct NewsArticleTitle {
    title: String,
    headline: String,
    location: String,
}

impl SummaryTitle for NewsArticleTitle {
    fn summarize_title(&self) -> String {
        format!("@{}", self.title)
    }
}

struct TweetTitle {
    title: String,
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl SummaryTitle for TweetTitle {
    fn summarize_title(&self) -> String {
        format!("@{}", self.title)
    }
}

fn notify(item: &impl SummaryTitle) {
    println!("{}", item.summarize());
}

fn returns_summarize() -> impl SummaryTitle {
    TweetTitle {
        title: String::from("title-test"),
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub fn test() {
    let news_article = NewsArticleTitle {
        title: "title-test".to_string(),
        headline: "headline-test".to_string(),
        location: "location".to_string()
    };

    println!("{}", news_article.summarize_title());
}

pub fn test_2() {
    let article = NewsArticleTitle {
        title: String::from("Breaking News"),
        headline: String::from("test head line"),
        location: String::from("Something important happened!"),
    };

    notify(&article);
}

pub fn test_3() {
    let t = returns_summarize();
    println!("{}", t.summarize_title());
}