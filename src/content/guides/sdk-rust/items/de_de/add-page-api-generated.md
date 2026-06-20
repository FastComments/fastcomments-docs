## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_api_page_data | models::CreateApiPageData | Ja |  |

## Antwort

Gibt zurück: [`AddPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_page_api_response.rs)

## Beispiel

[inline-code-attrs-start title = 'add_page Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let create_api_page_data: models::CreateApiPageData = models::CreateApiPageData {
    path: "news/article".to_string(),
    title: "Breaking: Market Rally".to_string(),
    url: Some("https://acme.example.com/news/market-rally".to_string()),
    meta_description: Some("Markets surge after earnings beat expectations".to_string()),
    tags: Some(vec!["finance".to_string(), "markets".to_string()]),
};

let params: AddPageParams = AddPageParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_api_page_data,
};

let response: AddPageApiResponse = add_page(&configuration, params).await?;
[inline-code-end]

---