## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| delete_comments | String | Ні |  |
| comment_delete_mode | String | Ні |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'delete_tenant_user Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantUserParams = DeleteTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        delete_comments: Some("true".to_string()),
        comment_delete_mode: Some("cascade".to_string()),
    };
    let resp: FlagCommentPublic200Response = delete_tenant_user(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---