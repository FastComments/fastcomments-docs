## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| large_internal_url_sanitized | String | 예 |  |

## 응답

반환: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/gif_get_large_response.rs)

## 예제

[inline-code-attrs-start title = 'get_gif_large 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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