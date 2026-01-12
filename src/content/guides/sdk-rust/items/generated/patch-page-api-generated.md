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
async fn run() -> Result<(), Error> {
    let params: PatchPageParams = PatchPageParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article-123"),
        update_api_page_data: models::UpdateApiPageData {
            title: Some(String::from("Breaking: Rust 1.80 Released")),
            path: String::from("/news/rust-1-80"),
            description: Some(String::from("Highlights and features in Rust 1.80")),
            is_published: Some(true),
            metadata: Some(std::collections::HashMap::from([
                (String::from("author"), String::from("Jane Doe")),
                (String::from("category"), String::from("technology")),
            ])),
        },
    };
    let response: PatchPageApiResponse = patch_page(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
