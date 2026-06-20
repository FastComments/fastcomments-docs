---
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| locale | String | Не |  |
| rating | String | Не |  |
| page | f64 | Не |  |

## Отговор

Връща: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_trending_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_gifs_trending'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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