---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| search | String | 是 |  |
| locale | String | 否 |  |
| rating | String | 否 |  |
| page | f64 | 否 |  |

## 响应

返回: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_search_response.rs)

## 示例

[inline-code-attrs-start title = 'get_gifs_search 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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