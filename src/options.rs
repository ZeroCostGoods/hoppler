// our struct for possible event posting options
#[derive(FromForm)]
pub struct PostEventsOptions {
    // specifies a username header we should pull from the request headers
    pub uh: Option<String>,
}