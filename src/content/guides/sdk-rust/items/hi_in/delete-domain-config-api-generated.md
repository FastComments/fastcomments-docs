## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| domain | String | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_domain_config_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'delete_domain_config उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = DeleteDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain: "news/article".to_string(),
    };
    let _response: DeleteDomainConfigResponse = delete_domain_config(configuration, params).await?;
    Ok(())
}
[inline-code-end]