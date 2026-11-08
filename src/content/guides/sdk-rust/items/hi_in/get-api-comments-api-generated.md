## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| page | f64 | नहीं |  |
| count | f64 | नहीं |  |
| text_search | String | नहीं |  |
| by_ip_from_comment | String | नहीं |  |
| filters | String | नहीं |  |
| search_filters | String | नहीं |  |
| sorts | String | नहीं |  |
| demo | bool | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_api_comments उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetApiCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some("rust".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        sorts: Some("date:desc".to_string()),
        demo: Some(false),
        sso: None,
    };
    let _response = get_api_comments(&configuration, params).await?;
    Ok(())
}
[inline-code-end]