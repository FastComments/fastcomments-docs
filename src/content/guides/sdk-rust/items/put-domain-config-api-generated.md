## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| domain_to_update | String | Yes |  |
| update_domain_config_params | models::UpdateDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_200_response.rs)

## Example

[inline-code-attrs-start title = 'put_domain_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_put_domain_config() -> Result<(), Error> {
    let params: PutDomainConfigParams = PutDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news.acme.com".to_string(),
        update_domain_config_params: models::UpdateDomainConfigParams {
            allow_comments: Some(true),
            moderation: Some("pre_moderation".to_string()),
            allowed_origins: Some(vec![
                "https://www.acme.com".to_string(),
                "https://news.acme.com".to_string(),
            ]),
            default_sort: Some("newest".to_string()),
        },
    };

    let updated: GetDomainConfig200Response = put_domain_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
