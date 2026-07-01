## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| page | f64 | 否 |  |
| count | f64 | 否 |  |
| text_search | String | 否 |  |
| by_ip_from_comment | String | 否 |  |
| filters | String | 否 |  |
| search_filters | String | 否 |  |
| sorts | String | 否 |  |
| demo | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳：[`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## 範例

[inline-code-attrs-start title = 'get_api_comments 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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