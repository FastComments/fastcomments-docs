## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| locale | String | Non |  |
| rating | String | Non |  |
| page | f64 | Non |  |

## Réponse

Renvoie : [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_trending_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_gifs_trending'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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