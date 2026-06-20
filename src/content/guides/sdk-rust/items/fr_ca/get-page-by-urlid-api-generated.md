## Paramètres

| Name | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |

## Réponse

Retourne: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_page_by_urlid'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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