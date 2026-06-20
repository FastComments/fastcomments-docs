---
Список страниц для арендатора. Используется настольным клиентом FChat для заполнения списка комнат.
Требуется, чтобы `enableFChat` был равен true в итоговой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются с учётом группового доступа запрашивающего пользователя.

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| cursor | String | Нет |  |
| limit | i32 | Нет |  |
| q | String | Нет |  |
| sort_by | models::PagesSortBy | Нет |  |
| has_comments | bool | Нет |  |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Пример

[inline-code-attrs-start title = 'get_pages_public Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---