## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| update_tenant_user_body | models::UpdateTenantUserBody | हाँ |  |
| update_comments | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_tenant_user उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            email: "jane.doe@example.com".to_string(),
            role: "editor".to_string(),
        },
        update_comments: Some("Promoted to editor".to_string()),
    };
    let _ = update_tenant_user(configuration, params).await?;
    Ok(())
}
[inline-code-end]