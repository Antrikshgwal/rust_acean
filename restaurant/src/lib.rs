pub trait Summary{
fn summarize(&self) ->String{
String::from("Read more..")
}
}

pub struct NewsArticle{
   pub author:String,
   pub headline:String,
   pub location:String,
   pub content:String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} by {}, from {}", self.headline, self.author, self.location)
    // }
}

pub struct SocialPost {
    pub username: String,
    pub content:String,
    pub reply: String,
    pub repost:String,
}
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

//  Traits can be returned by functions

fn returns_summarizable() => impl Summary {
    SocialPost{
        username : String::from("Justin drake"),
        content: String::("Bitcoin is doomed"),
        reply:false,
        repost:false,
    }
}