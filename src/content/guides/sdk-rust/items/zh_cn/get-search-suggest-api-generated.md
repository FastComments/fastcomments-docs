## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| text_search | String | No |  |
| sso | String | No |  |

## 响应

返回：[`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_suggest_response.rs)

## 示例

[inline-code-attrs-start title = 'get_search_suggest 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchSuggestParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("news/article".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: ModerationSuggestResponse = get_search_suggest(configuration, params).await?;
    Ok(())
}
[inline-code-end]