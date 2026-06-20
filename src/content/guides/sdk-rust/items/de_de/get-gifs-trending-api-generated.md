## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| locale | String | Nein |  |
| rating | String | Nein |  |
| page | f64 | Nein |  |

## Antwort

Gibt zurück: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_trending_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_gifs_trending Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_trending_gifs() -> Result<GetGifsTrendingResponse, Error> {
    let params: GetGifsTrendingParams = GetGifsTrendingParams {
        tenant_id: String::from("acme-corp-tenant"),
        locale: Some(String::from("en-US")),
        rating: Some(String::from("pg-13")),
        page: Some(1.0),
    };
    let trending: GetGifsTrendingResponse = get_gifs_trending(&configuration, params).await?;
    Ok(trending)
}
[inline-code-end]

---