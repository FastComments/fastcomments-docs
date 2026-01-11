## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_apisso_user_data | models::CreateApissoUserData | Yes |  |

## Response

Returns: [`AddSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_sso_user_api_response.rs)

## Example

[inline-code-attrs-start title = 'add_sso_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: AddSsoUserParams = AddSsoUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_apisso_user_data: models::CreateApissoUserData {
        user_id: "jdoe".to_string(),
        email: "jdoe@acme.com".to_string(),
        display_name: Some("John Doe".to_string()),
        roles: Some(vec!["editor".to_string(), "contributor".to_string()]),
        external_id: Some("ldap:uid=12345,ou=users,dc=acme,dc=com".to_string()),
        avatar_url: Some("https://acme.com/avatars/jdoe.png".to_string()),
        metadata: Some(std::collections::HashMap::from([
            ("department".to_string(), "Engineering".to_string()),
            ("location".to_string(), "NYC".to_string()),
        ])),
        verified: Some(true),
    },
};
let response: AddSsoUserApiResponse = add_sso_user(&configuration, params).await?;
[inline-code-end]
