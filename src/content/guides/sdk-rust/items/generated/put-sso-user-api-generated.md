## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_apisso_user_data | models::UpdateApissoUserData | Yes |  |
| update_comments | bool | No |  |

## Response

Returns: [`PutSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_sso_user_api_response.rs)

## Example

[inline-code-attrs-start title = 'put_sso_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PutSsoUserParams = PutSsoUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "user-12345".to_string(),
    update_apisso_user_data: models::UpdateApissoUserData {
        email: "jane.doe@acme.com".to_string(),
        display_name: "Jane Doe".to_string(),
        external_id: "sso-9876".to_string(),
        roles: vec!["editor".to_string()],
    },
    update_comments: Some(true),
};

let response: PutSsoUserApiResponse = put_sso_user(&configuration, params).await?;
[inline-code-end]
