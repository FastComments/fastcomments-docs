## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| update_tenant_body | models::UpdateTenantBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_tenant उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = UpdateTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "tenant-001".to_string(),
        update_tenant_body: UpdateTenantBody {
            description: Some("Primary tenant for Acme Corp".to_string()),
            ..Default::default()
        },
    };
    let _ = update_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]