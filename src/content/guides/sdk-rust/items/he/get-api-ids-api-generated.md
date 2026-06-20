## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| text_search | String | לא |  |
| by_ip_from_comment | String | לא |  |
| filters | String | לא |  |
| search_filters | String | לא |  |
| after_id | String | לא |  |
| demo | bool | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_api_ids'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<ModerationApiGetCommentIdsResponse, Error> {
    let params: GetApiIdsParams = GetApiIdsParams {
        text_search: Some("climate policy debate".to_string()),
        by_ip_from_comment: Some("198.51.100.23".to_string()),
        filters: Some("status:approved,section:opinion".to_string()),
        search_filters: Some("author:guest".to_string()),
        after_id: Some("cmt_000123abc".to_string()),
        demo: Some(false),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let response: ModerationApiGetCommentIdsResponse = get_api_ids(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---