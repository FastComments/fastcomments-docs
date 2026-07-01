## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_apisso_user_data | models::CreateApissoUserData | Yes |  |

## レスポンス

戻り値: [`AddSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_sso_user_api_response.rs)

## 例

[inline-code-attrs-start title = 'add_sso_user の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let user_data = models::CreateApissoUserData {
        username: "jdoe".to_string(),
        email: "jdoe@acme.com".to_string(),
        display_name: Some("John Doe".to_string()),
        is_active: Some(true),
    };
    let params = AddSsoUserParams {
        tenant_id: "acme-corp".to_string(),
        create_apisso_user_data: user_data,
    };
    let _response = add_sso_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]