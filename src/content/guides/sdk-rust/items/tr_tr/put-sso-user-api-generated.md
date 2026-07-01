## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| update_apisso_user_data | models::UpdateApissoUserData | Evet |  |
| update_comments | bool | Hayır |  |

## Response

Döndürür: [`PutSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_sso_user_api_response.rs)

## Example

[inline-code-attrs-start title = 'put_sso_user Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let update_data = UpdateApissoUserData {
        email: "jane.doe@example.com".to_string(),
        display_name: "Jane Doe".to_string(),
    };
    let params = PutSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        update_apisso_user_data: update_data,
        update_comments: Some(true),
    };
    let _response = put_sso_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]