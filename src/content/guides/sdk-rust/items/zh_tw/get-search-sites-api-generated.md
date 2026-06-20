## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| value | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_site_search_response.rs)

## 範例

[inline-code-attrs-start title = 'get_search_sites 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_search() -> Result<(), Error> {
    let params = GetSearchSitesParams {
        value: Some("news/article".to_string()),
        sso: Some("acme-sso-provider".to_string()),
    };
    let response: ModerationSiteSearchResponse = get_search_sites(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---