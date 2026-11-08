## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| domain | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_domain_config उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = GetDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain: "news/article".to_string(),
        locale: Some("en-US".to_string()),
    };
    let _response = get_domain_config(&config, params).await?;
    Ok(())
}
[inline-code-end]