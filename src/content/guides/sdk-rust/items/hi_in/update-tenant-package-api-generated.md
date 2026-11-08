## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| update_tenant_package_body | models::UpdateTenantPackageBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_tenant_package उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update(configuration: &configuration::Configuration) -> Result<(), Error> {
    let body = models::UpdateTenantPackageBody {
        plan: Some("enterprise".to_string()),
        renewal_date: Some("2024-12-31".to_string()),
        ..Default::default()
    };
    let params = UpdateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "pkg-2024".to_string(),
        update_tenant_package_body: body,
    };
    let _: ApiEmptyResponse = update_tenant_package(configuration, params).await?;
    Ok(())
}
[inline-code-end]