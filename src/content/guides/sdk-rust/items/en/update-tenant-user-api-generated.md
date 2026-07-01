## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Yes |  |
| update_comments | String | No |  |

## Response

Returns: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Example

[inline-code-attrs-start title = 'update_tenant_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
