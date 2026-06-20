## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| value | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_site_search_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_search_sites Örnek'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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