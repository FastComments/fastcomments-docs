## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| locale | String | Ne |  |
| rating | String | Ne |  |
| page | f64 | Ne |  |

## Odgovor

Vraća: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_trending_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_gifs_trending'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_trending_gifs(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetGifsTrendingParams {
        tenant_id: "acme-corp-tenant".to_string(),
        locale: Some("en-US".to_string()),
        rating: Some("pg".to_string()),
        page: Some(1.0),
    };
    let _response = get_gifs_trending(configuration, params).await?;
    Ok(())
}
[inline-code-end]