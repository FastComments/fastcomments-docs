## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| large_internal_url_sanitized | String | כן |  |

## תגובה

מחזיר: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/gif_get_large_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_gif_large'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GifGetLargeResponse, Error> {
    let params: GetGifLargeParams = GetGifLargeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        large_internal_url_sanitized: "gifs/news/article/welcome-gif".to_string(),
        referrer: Some("https://news.example.com/article/123".to_string()),
    };
    let response: GifGetLargeResponse = get_gif_large(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---