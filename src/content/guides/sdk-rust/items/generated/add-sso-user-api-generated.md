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
        username: "jane.doe".to_string(),
        email: "jane.doe@acme-corp.com".to_string(),
        display_name: Some("Jane Doe".to_string()),
        external_id: Some("sso-12345".to_string()),
        avatar_url: Some("https://acme-corp.com/avatars/jane.jpg".to_string()),
        roles: Some(vec!["author".to_string(), "moderator".to_string()]),
        metadata: Some({
            let mut m = std::collections::HashMap::new();
            m.insert("team".to_string(), "editorial".to_string());
            m.insert("tier".to_string(), "premium".to_string());
            m
        }),
    },
};

let response: AddSsoUserApiResponse = add_sso_user(&configuration, params).await?;
[inline-code-end]
