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
async fn example() -> Result<(), Error> {
    let params: AddPageParams = AddPageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_page_data: models::CreateApiPageData {
            page_id: Some("news/article-2026-01".to_string()),
            url: "https://blog.acme.com/news/2026/01/article".to_string(),
            title: Some("Acme Product Update - January 2026".to_string()),
            path: Some("/news/2026/01/article".to_string()),
            language: Some("en-US".to_string()),
            tags: Some(vec!["product".to_string(), "release".to_string()]),
            allow_anonymous: Some(false),
        },
    };

    let response: AddPageApiResponse = add_page(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
