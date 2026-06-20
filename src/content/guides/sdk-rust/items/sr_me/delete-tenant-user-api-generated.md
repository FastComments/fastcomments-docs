## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| delete_comments | String | Не |  |
| comment_delete_mode | String | Не |  |

## Одговор

Враћа: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Примјер

[inline-code-attrs-start title = 'delete_tenant_user Примјер'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteTenantUserParams = DeleteTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-8421".to_string(),
        delete_comments: Some("yes".to_string()),
        comment_delete_mode: Some("permanent".to_string()),
    };
    let _response: ApiEmptyResponse = delete_tenant_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]