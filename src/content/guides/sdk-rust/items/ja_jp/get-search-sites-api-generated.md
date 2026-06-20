## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| value | String | 任意 |  |
| sso | String | 任意 |  |

## レスポンス

戻り値: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_site_search_response.rs)

## 例

[inline-code-attrs-start title = 'get_search_sites の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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