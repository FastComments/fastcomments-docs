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
async fn update_sso_user() -> Result<PutSsoUserApiResponse, Error> {
    let params: PutSsoUserParams = PutSsoUserParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("user-12345"),
        update_apisso_user_data: models::UpdateApissoUserData {
            email: String::from("jane.doe@acme.com"),
            name: String::from("Jane Doe"),
            external_id: String::from("sso-67890"),
            provider: String::from("saml"),
            roles: vec![String::from("editor")],
        },
        update_comments: Some(true),
    };
    let response: PutSsoUserApiResponse = put_sso_user(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
