Колишні коментатори на сторінці, які НЕ зараз в мережі. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити розділ "Учасники".
Посторінкова пагінація курсором по commenterName: сервер обходить частковий {tenantId, urlId, commenterName}
індекс починаючи від afterName вперед через $gt, без витрат $skip.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| after_name | String | Ні |  |
| after_user_id | String | Ні |  |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---