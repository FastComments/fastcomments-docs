## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| text_search | String | 否 |  |
| by_ip_from_comment | String | 否 |  |
| filter | String | 否 |  |
| search_filters | String | 否 |  |
| demo | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## 範例

[inline-code-attrs-start title = 'get_count 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_count() -> Result<ModerationApiCountCommentsResponse, Error> {
    let params: GetCountParams = GetCountParams {
        text_search: Some("breaking election coverage".to_string()),
        by_ip_from_comment: Some("203.0.113.45".to_string()),
        filter: Some("status:approved".to_string()),
        search_filters: Some("section:politics tag:analysis".to_string()),
        demo: Some(false),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let response: ModerationApiCountCommentsResponse = get_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---