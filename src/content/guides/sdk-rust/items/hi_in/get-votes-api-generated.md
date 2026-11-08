## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_votes उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetVotesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        limit: Some(100),
    };
    let _response: GetVotesResponse = get_votes(configuration, params).await?;
    Ok(())
}
[inline-code-end]