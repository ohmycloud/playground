pub trait Summary {
    fn summarize(&self) -> String;
}
  
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
  
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
  
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}
  
impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}, {}", self.username, self.content)
    }
}

// trait as function argument
pub fn notify(item: impl Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}
  
fn main() {
      let tweet = Tweet {
        username: String::from("Larry Wall"),
        content: "发明了 Perl 6 语言, 后来改名为 Raku 了".to_string(),
        reply: false,
        retweet: true,
      };
  
      let article = NewsArticle {
        headline: String::from("19 万公里!"),
        location: "月球".to_string(),
        author: "上帝之手".to_string(),
        content: String::from("月球向地球靠近19万公里"),
      };
      println!("1 new tweet: {}", tweet.summarize());
      println!("头条新闻: {}, {}, {} - {}", article.headline, article.content, article.location, article.author);

      println!("1 new tweet: {:#?}", notify(tweet));
      println!("another new tweet: {:#?}", notify(article));
  }
  