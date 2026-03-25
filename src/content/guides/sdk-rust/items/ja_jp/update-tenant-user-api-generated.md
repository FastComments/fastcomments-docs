## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| update_tenant_user_body | models::UpdateTenantUserBody | はい |  |
| update_comments | String | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 例

[inline-code-attrs-start title = 'update_tenant_user の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_tenant_user(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateTenantUserParams = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9f".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            email: Some("jane.doe@acme.com".to_string()),
            display_name: Some("Jane Doe".to_string()),
            username: Some("jdoe".to_string()),
            role: Some("moderator".to_string()),
        },
        update_comments: Some("Promoted to moderator to handle flagged comments".to_string()),
    };
    let response = update_tenant_user(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---