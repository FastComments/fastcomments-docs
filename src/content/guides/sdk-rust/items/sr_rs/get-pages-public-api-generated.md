Листа страница за тенант. Користи се од стране FChat десктоп клијента за попуњавање његове листе соба.
Захтева да `enableFChat` буде true у решеној прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO филтрирају се према приступу групе корисника који подноси захтев.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| cursor | String | Не |  |
| limit | i32 | Не |  |
| q | String | Не |  |
| sort_by | models::PagesSortBy | Не |  |
| has_comments | bool | Не |  |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

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