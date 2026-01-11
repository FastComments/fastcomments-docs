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
async fn update_domain() -> Result<GetDomainConfig200Response, Error> {
    let params: PatchDomainConfigParams = PatchDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news/articles".to_string(),
        patch_domain_config_params: models::PatchDomainConfigParams {
            allowed_origins: Some(vec![
                "https://www.acme.com".to_string(),
                "https://editor.acme.com".to_string(),
            ]),
            moderation_mode: Some("pre_moderation".to_string()),
            max_comment_length: Some(2000),
            enable_cors: Some(true),
            ..Default::default()
        },
    };
    let updated: GetDomainConfig200Response = patch_domain_config(&configuration, params).await?;
    Ok(updated)
}
[inline-code-end]
