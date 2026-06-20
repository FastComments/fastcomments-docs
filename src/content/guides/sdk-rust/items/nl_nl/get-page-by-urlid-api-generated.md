## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |

## Respons

Geeft terug: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_page_by_urlid Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_page() -> Result<GetPageByUrlidApiResponse, Error> {
    let params: GetPageByUrlidParams = GetPageByUrlidParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/how-to-build-an-api".to_string(),
        locale: Some("en-US".to_string()),
    };
    let page: GetPageByUrlidApiResponse = get_page_by_urlid(&configuration, params).await?;
    Ok(page)
}
[inline-code-end]

---