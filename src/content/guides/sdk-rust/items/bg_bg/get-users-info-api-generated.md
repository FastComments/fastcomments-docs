Масова информация за потребители за даден tenant. При подадени userIds връща информация за показване от User / SSOUser.
Използва се от коментарния уиджет за допълване на потребители, които току-що се появиха чрез събитие за присъствие.
Без контекст на страница: поверителността се прилага еднакво (личните профили се маскират).

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| ids | String | Да |  |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]