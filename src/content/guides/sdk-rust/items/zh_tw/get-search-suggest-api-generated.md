## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| text_search | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_suggest_response.rs)

## 範例

[inline-code-attrs-start title = 'get_search_suggest 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_suggest() -> Result<(), Error> {
    let params: GetSearchSuggestParams = GetSearchSuggestParams {
        text_search: Some("news/article: presidential debate highlights".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let suggestion: ModerationSuggestResponse = get_search_suggest(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---