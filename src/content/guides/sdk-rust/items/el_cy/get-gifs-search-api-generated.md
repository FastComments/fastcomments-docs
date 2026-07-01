## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Yes |  |
| search | String | Yes |  |
| locale | String | No |  |
| rating | String | No |  |
| page | f64 | No |  |

## Απόκριση

Επιστρέφει: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_search_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'get_gifs_search Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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