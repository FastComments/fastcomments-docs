## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'delete_tenant_package उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "premium-plan".to_string(),
        force: Some(true),
    };
    delete_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]