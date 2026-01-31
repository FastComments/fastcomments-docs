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
        email: "jane.doe@acme.com".to_string(),
        name: Some("Jane Doe".to_string()),
        external_id: Some("okta|00u1abcd2EFGH3IJ4".to_string()),
        roles: Some(vec!["moderator".to_string(), "author".to_string()]),
        avatar_url: Some("https://cdn.acme-corp.com/avatars/jane.jpg".to_string()),
        metadata: Some(vec![("department".to_string(), "Editorial".to_string())].into_iter().collect()),
    },
};
let response: AddSsoUserApiResponse = add_sso_user(&configuration, params).await?;
[inline-code-end]
