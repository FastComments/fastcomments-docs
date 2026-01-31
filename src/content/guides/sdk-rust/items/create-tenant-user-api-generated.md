## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_tenant_user_body | models::CreateTenantUserBody | Yes |  |

## Response

Returns: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_user_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_tenant_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateTenantUserParams = CreateTenantUserParams {
        tenant_id: "acme-news-tenant".to_string(),
        create_tenant_user_body: models::CreateTenantUserBody {
            username: "alice.johnson".to_string(),
            email: "alice.johnson@acme.com".to_string(),
            display_name: Some("Alice Johnson".to_string()),
            roles: Some(vec!["moderator".to_string(), "editor".to_string()]),
            is_admin: Some(false),
            locale: Some("en-US".to_string()),
        },
    };
    let response: CreateTenantUser200Response = create_tenant_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
