## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_api_page_data | models::CreateApiPageData | Yes |  |

## Response

Returns: [`AddPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_page_api_response.rs)

## Example

[inline-code-attrs-start title = 'add_page Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn create_page() -> Result<AddPageApiResponse, Error> {
    let params: AddPageParams = AddPageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_page_data: models::CreateApiPageData {
            path: "news/article".to_string(),
            title: "Rust 2.0 Released".to_string(),
            url: Some("https://acme.example/news/rust-2".to_string()),
            allow_comments: Some(true),
            tags: Some(vec!["rust".to_string(), "release".to_string()]),
            metadata: Some(std::collections::HashMap::from([
                ("author".to_string(), "Jane Doe".to_string()),
                ("section".to_string(), "technology".to_string())
            ])),
            published: Some(true)
        }
    };
    let response: AddPageApiResponse = add_page(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
