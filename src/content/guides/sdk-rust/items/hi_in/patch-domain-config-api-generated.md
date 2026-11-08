## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| domain_to_update | String | हाँ |  |
| patch_domain_config_params | models::PatchDomainConfigParams | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_domain_config_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'patch_domain_config उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = PatchDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news/article".to_string(),
        patch_domain_config_params: models::PatchDomainConfigParams {
            enable_comments: Some(true),
            theme: Some("dark".to_string()),
        },
    };
    let _response = patch_domain_config(&config, params).await?;
    Ok(())
}
[inline-code-end]