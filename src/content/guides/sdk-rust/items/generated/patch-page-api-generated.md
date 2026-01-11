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
let params: PatchPageParams = PatchPageParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "news/article".to_string(),
    update_api_page_data: models::UpdateApiPageData {
        title: Some("Breaking: Rust adoption surges".to_string()),
        slug: Some("news/article".to_string()),
        content: Some("<p>Rust is seeing increased adoption across startups.</p>".to_string()),
        published: Some(true),
        tags: Some(vec!["rust".to_string(), "technology".to_string()]),
        metadata: Some(serde_json::json!({"author":"Jane Doe","section":"News"})),
    },
};
let updated_page: PatchPageApiResponse = patch_page(&configuration, params).await?;
[inline-code-end]
