## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| update_apisso_user_data | models::UpdateApissoUserData | כן |  |
| update_comments | bool | לא |  |

## תשובה

מחזיר: [`PutSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_sso_user_api_response.rs)

## דוגמה

[inline-code-attrs-start title = 'put_sso_user דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---