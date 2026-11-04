Коментатори, які раніше залишали коментарі на сторінці, але зараз НЕ в мережі. Відсортовані за displayName.
Використовуйте це після вичерпання /users/online для відображення розділу «Учасники».
Курсорна пагінація за commenterName: сервер проходить частковим індексом {tenantId, urlId, commenterName}
від afterName вперед за допомогою $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Обов'язковий | Опис |
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