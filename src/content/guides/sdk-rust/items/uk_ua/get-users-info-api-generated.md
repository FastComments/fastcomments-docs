Масова інформація про користувачів для тенанта. За заданими userIds повертає інформацію для відображення з User / SSOUser.
Використовується віджетом коментарів для збагачення користувачів, які щойно з'явилися через подію присутності.
Без контексту сторінки: приватність застосовується однаково (приватні профілі маскуються).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| ids | String | Так |  |

## Відповідь

Повертає: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]