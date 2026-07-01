## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| value | String | No |  |
| sso | String | No |  |

## Odgovor

Vraća: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_site_search_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_search_sites Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetSearchSitesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("news/article".to_string()),
        sso: Some("sso-token-abc".to_string()),
    };
    let _response = get_search_sites(&config, params).await?;
    Ok(())
}
[inline-code-end]