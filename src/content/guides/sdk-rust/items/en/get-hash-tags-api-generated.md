## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| page | f64 | No |  |

## Response

Returns: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_hash_tags_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_hash_tags Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_hash_tags() -> Result<GetHashTags200Response, Error> {
    let params: GetHashTagsParams = GetHashTagsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0_f64),
    };
    let response: GetHashTags200Response = get_hash_tags(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
