## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_tenant उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        include_billing: Some(true),
    };
    let _response: GetTenantResponse = get_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]