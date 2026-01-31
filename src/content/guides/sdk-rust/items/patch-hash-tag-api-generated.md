## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | String | Yes |  |
| tenant_id | String | No |  |
| update_hash_tag_body | models::UpdateHashTagBody | No |  |

## Response

Returns: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_hash_tag_200_response.rs)

## Example

[inline-code-attrs-start title = 'patch_hash_tag Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_patch() -> Result<PatchHashTag200Response, Error> {
    let params: PatchHashTagParams = PatchHashTagParams {
        tag: "news/article".to_string(),
        tenant_id: Some("acme-corp-tenant".to_string()),
        update_hash_tag_body: Some(models::UpdateHashTagBody::default()),
    };
    let response: PatchHashTag200Response = patch_hash_tag(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
