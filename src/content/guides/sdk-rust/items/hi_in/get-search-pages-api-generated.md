## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| value | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_page_search_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_search_pages उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetSearchPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("news/article".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let response: ModerationPageSearchResponse = get_search_pages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]