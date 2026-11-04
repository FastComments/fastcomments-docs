## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| text_search | String | 否 |  |
| by_ip_from_comment | String | 否 |  |
| filters | String | 否 |  |
| search_filters | String | 否 |  |
| after_id | String | 否 |  |
| demo | bool | 否 |  |
| sso | String | 否 |  |

## 回應

Returns: [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## 範例

[inline-code-attrs-start title = 'get_api_ids 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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