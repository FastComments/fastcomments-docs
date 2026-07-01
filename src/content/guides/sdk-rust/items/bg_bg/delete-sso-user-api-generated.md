## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| delete_comments | bool | No |  |
| comment_delete_mode | String | No |  |

## Отговор

Връща: [`DeleteSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_sso_user_api_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_sso_user Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-42".to_string(),
        delete_comments: Some(true),
        comment_delete_mode: Some("soft".to_string()),
    };
    let _response: DeleteSsoUserApiResponse = delete_sso_user(&config, params).await?;
    Ok(())
}
[inline-code-end]