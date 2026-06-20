## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | String | Yes |  |
| tenant_id | String | No |  |
| update_hash_tag_body | models::UpdateHashTagBody | No |  |

## Response

Returns: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## Example

[inline-code-attrs-start title = 'patch_hash_tag Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let cfg: &configuration::Configuration = &configuration;
let body: models::UpdateHashTagBody = Default::default();
let params: PatchHashTagParams = PatchHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    update_hash_tag_body: Some(body),
};
let response: UpdateHashTagResponse = patch_hash_tag(cfg, params).await?;
[inline-code-end]
