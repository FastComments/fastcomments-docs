---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|------|
| tenant_id | String | 是 |  |
| value | String | 否 |  |
| sso | String | 否 |  |

## 回應

返回：[`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_page_search_response.rs)

## 範例

[inline-code-attrs-start title = 'get_search_pages 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---