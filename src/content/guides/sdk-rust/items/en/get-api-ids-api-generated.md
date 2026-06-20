## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| text_search | String | No |  |
| by_ip_from_comment | String | No |  |
| filters | String | No |  |
| search_filters | String | No |  |
| after_id | String | No |  |
| demo | bool | No |  |
| sso | String | No |  |

## Response

Returns: [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## Example

[inline-code-attrs-start title = 'get_api_ids Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
