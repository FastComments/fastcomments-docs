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
async fn run() -> Result<(), Error> {
    let params: AddPageParams = AddPageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_page_data: models::CreateApiPageData {
            path: "news/article/2025-election".to_string(),
            title: "2025 Election Analysis".to_string(),
            url: Some("https://acme.example.com/news/2025-election".to_string()),
            description: Some("In-depth analysis of the 2025 election".to_string()),
            tags: Some(vec!["politics".to_string(), "election".to_string()]),
            is_active: Some(true),
        },
    };
    let response: AddPageApiResponse = add_page(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
