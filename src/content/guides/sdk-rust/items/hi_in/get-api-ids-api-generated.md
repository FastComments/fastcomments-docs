## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| text_search | String | नहीं |  |
| by_ip_from_comment | String | नहीं |  |
| filters | String | नहीं |  |
| search_filters | String | नहीं |  |
| after_id | String | नहीं |  |
| demo | bool | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_api_ids उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetApiIdsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("breaking news".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: None,
        after_id: None,
        demo: Some(false),
        sso: Some("sso-token".to_string()),
    };
    let _response = get_api_ids(&config, params).await?;
    Ok(())
}
[inline-code-end]