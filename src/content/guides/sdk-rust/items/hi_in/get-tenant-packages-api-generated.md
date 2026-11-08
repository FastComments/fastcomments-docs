## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_tenant_packages उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetTenantPackagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(5.0),
    };
    let _resp = get_tenant_packages(&config, params).await?;
    Ok(())
}
[inline-code-end]