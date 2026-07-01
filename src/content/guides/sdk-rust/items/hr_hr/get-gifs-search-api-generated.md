## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| search | String | Da |  |
| locale | String | Ne |  |
| rating | String | Ne |  |
| page | f64 | Ne |  |

## Odgovor

Vraća: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_search_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer get_gifs_search'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_gifs(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetGifsSearchParams {
        tenant_id: "acme-corp-tenant".to_string(),
        search: "funny cats".to_string(),
        locale: Some("en-US".to_string()),
        rating: Some("pg".to_string()),
        page: Some(1.0),
    };
    let _response = get_gifs_search(config, params).await?;
    Ok(())
}
[inline-code-end]