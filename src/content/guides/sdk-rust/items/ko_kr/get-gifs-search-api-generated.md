## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| search | String | Yes |  |
| locale | String | No |  |
| rating | String | No |  |
| page | f64 | No |  |

## 응답

반환: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_search_response.rs)

## 예제

[inline-code-attrs-start title = 'get_gifs_search 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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