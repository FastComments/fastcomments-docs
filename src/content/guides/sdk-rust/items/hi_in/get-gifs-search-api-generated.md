## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| search | String | हाँ |  |
| locale | String | नहीं |  |
| rating | String | नहीं |  |
| page | f64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_search_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_gifs_search उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_gifs(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetGifsSearchParams {
        tenant_id: "acme-corp-tenant".to_string(),
        search: "funny cats".to_string(),
        locale: Some("en-US".to_string()),
        rating: Some("pg".to_string()),
        page: Some(1.0),
    };
    let _response = get_gifs_search(config, params).await?;
    Ok(())
}
[inline-code-end]