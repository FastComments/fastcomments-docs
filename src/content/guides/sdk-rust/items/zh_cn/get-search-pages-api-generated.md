## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| value | String | No |  |
| sso | String | No |  |

## 响应

返回: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_page_search_response.rs)

## 示例

[inline-code-attrs-start title = 'get_search_pages 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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