## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| domain_to_update | String | Yes |  |
| patch_domain_config_params | models::PatchDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_200_response.rs)

## Example

[inline-code-attrs-start title = 'patch_domain_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PatchDomainConfigParams = PatchDomainConfigParams {
    tenant_id: String::from("acme-corp-tenant"),
    domain_to_update: String::from("news/article"),
    patch_domain_config_params: models::PatchDomainConfigParams {
        allow_comments: Some(true),
        moderation: Some(String::from("pre-moderation")),
        max_comment_length: Some(2048),
        custom_css: Some(String::from(".fc-comment{font-size:13px;color:#222}")),
    },
};
let updated: GetDomainConfig200Response = patch_domain_config(&configuration, params).await?;
[inline-code-end]
