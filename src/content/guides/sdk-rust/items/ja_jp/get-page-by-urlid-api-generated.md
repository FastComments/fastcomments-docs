## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## レスポンス

返却: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## 例

[inline-code-attrs-start title = 'get_page_by_urlid 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetPageByUrlidParams {
        tenant_id: "acme-corp-tenant".into(),
        url_id: "news/article".into(),
    };
    let _response = get_page_by_urlid(&config, params).await?;
    Ok(())
}
[inline-code-end]

---