## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_tenant_user_body | models::UpdateTenantUserBody | 예 |  |
| update_comments | String | 아니요 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'update_tenant_user 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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