## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| text_search | String | No |  |
| by_ip_from_comment | String | No |  |
| filter | String | No |  |
| search_filters | String | No |  |
| demo | bool | No |  |
| sso | String | No |  |

## 响应

返回: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## 示例

[inline-code-attrs-start title = 'get_count 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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