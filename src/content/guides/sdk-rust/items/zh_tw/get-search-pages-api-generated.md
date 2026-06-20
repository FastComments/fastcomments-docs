## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| value | String | 否 |  |
| sso | String | 否 |  |

## 回應

Returns: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_page_search_response.rs)

## 範例

[inline-code-attrs-start title = 'get_search_pages 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetSearchPagesParams {
        value: Some("news/article/world/2026-summit".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let moderation_response: ModerationPageSearchResponse =
        get_search_pages(&configuration, params).await?;
    let _status: ApiStatus = moderation_response.status;
    Ok(())
}
[inline-code-end]

---