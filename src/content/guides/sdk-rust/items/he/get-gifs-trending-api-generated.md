## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| locale | String | לא |  |
| rating | String | לא |  |
| page | f64 | לא |  |

## תגובה

מחזיר: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_trending_response.rs)

## דוגמה

[inline-code-attrs-start title = 'get_gifs_trending דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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