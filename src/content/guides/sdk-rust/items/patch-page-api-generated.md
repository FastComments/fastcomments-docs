## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_api_page_data | models::UpdateApiPageData | Yes |  |

## Response

Returns: [`PatchPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_page_api_response.rs)

## Example

[inline-code-attrs-start title = 'patch_page Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_patch() -> Result<PatchPageApiResponse, Error> {
    let params: PatchPageParams = PatchPageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/acme-launch-2026".to_string(),
        update_api_page_data: models::UpdateApiPageData {
            title: Some("Acme Launch 2026: Successful Orbital Test".to_string()),
            slug: Some("acme-launch-2026".to_string()),
            description: Some("Coverage of Acme's successful orbital test flight.".to_string()),
            is_published: Some(true),
            tags: Some(vec!["press".to_string(), "launch".to_string(), "acme".to_string()]),
            metadata: Some({
                let mut m = std::collections::HashMap::new();
                m.insert("author".to_string(), "Jane Doe".to_string());
                m.insert("region".to_string(), "us-west".to_string());
                m
            }),
        },
    };
    let updated_page: PatchPageApiResponse = patch_page(&configuration, params).await?;
    Ok(updated_page)
}
[inline-code-end]
