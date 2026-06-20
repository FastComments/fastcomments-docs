Изброява страници за наемател. Използва се от настолния клиент FChat за попълване на списъка със стаи. Изисква `enableFChat` да бъде true в разрешената персонализирана конфигурация за всяка страница. Страниците, които изискват SSO, се филтрират въз основа на груповия достъп на потребителя, който прави заявката.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| cursor | String | Не |  |
| limit | i32 | Не |  |
| q | String | Не |  |
| sort_by | models::PagesSortBy | Не |  |
| has_comments | bool | Не |  |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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