Минали коментатори на страницата, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Members".
Пагинация с курсор по commenterName: сървърът обхожда частичния индекс {tenantId, urlId, commenterName} от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| after_name | String | Не |  |
| after_user_id | String | Не |  |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]