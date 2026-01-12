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
async fn run() -> Result<(), Error> {
    let params: PatchDomainConfigParams = PatchDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "blog.acme.com".to_string(),
        patch_domain_config_params: models::PatchDomainConfigParams {
            moderation_enabled: Some(true),
            allowed_origins: Some(vec!["https://blog.acme.com".to_string()]),
            custom_css: Some(".fc-comment{font-size:14px}".to_string()),
            notify_on_new_comment: Some(true),
            ..Default::default()
        },
    };
    let resp: GetDomainConfig200Response = patch_domain_config(&configuration, params).await?;
    println!("{:#?}", resp);
    Ok(())
}
[inline-code-end]
