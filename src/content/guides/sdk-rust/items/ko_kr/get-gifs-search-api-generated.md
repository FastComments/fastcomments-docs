## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| search | String | 예 |  |
| locale | String | 아니오 |  |
| rating | String | 아니오 |  |
| page | f64 | 아니오 |  |

## 응답

반환: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_search_response.rs)

## 예제

[inline-code-attrs-start title = 'get_gifs_search 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_gif_search() -> Result<(), Error> {
    let params: GetGifsSearchParams = GetGifsSearchParams {
        tenant_id: "acme-corp-tenant".to_string(),
        search: "breaking news".to_string(),
        locale: Some("en-US".to_string()),
        rating: Some("pg-13".to_string()),
        page: Some(1.0),
    };
    let response: GetGifsSearchResponse = get_gifs_search(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---