Прошлые комментаторы на странице, которые в настоящий момент НЕ в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online, чтобы отобразить раздел «Members».
Постраничная пагинация по commenterName: сервер проходит по частичному {tenantId, urlId, commenterName} индексу начиная после afterName вперёд через $gt, без затрат $skip.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| after_name | String | Нет |  |
| after_user_id | String | Нет |  |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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