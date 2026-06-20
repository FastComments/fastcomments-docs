Перелік сторінок для тенанта. Використовується настільним клієнтом FChat для заповнення його списку кімнат.
Потребує, щоб `enableFChat` було true у визначеній кастомній конфігурації для кожної сторінки.
Сторінки, що вимагають SSO, фільтруються відповідно до групового доступу користувача, який робить запит.

## Параметри

| Назва | Type | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| cursor | String | Ні |  |
| limit | i32 | Ні |  |
| q | String | Ні |  |
| sort_by | models::PagesSortBy | Ні |  |
| has_comments | bool | Ні |  |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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