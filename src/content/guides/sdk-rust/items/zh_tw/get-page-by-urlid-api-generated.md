## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |

## 回應

Returns: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## 範例

[inline-code-attrs-start title = 'get_page_by_urlid 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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