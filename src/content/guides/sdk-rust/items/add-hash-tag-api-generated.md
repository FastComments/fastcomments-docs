## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | No |  |
| create_hash_tag_body | models::CreateHashTagBody | No |  |

## Response

Returns: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_hash_tag_200_response.rs)

## Example

[inline-code-attrs-start title = 'add_hash_tag Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_add_hash_tag(configuration: &configuration::Configuration) -> Result<AddHashTag200Response, Error> {
    let params: AddHashTagParams = AddHashTagParams {
        tenant_id: Some("acme-corp-tenant".to_string()),
        create_hash_tag_body: Some(models::CreateHashTagBody {
            name: "breaking-news".to_string(),
            slug: Some("news/breaking".to_string()),
            description: Some("Posts about urgent breaking news events".to_string()),
            is_active: Some(true),
        }),
    };

    let response: AddHashTag200Response = add_hash_tag(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
