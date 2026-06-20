Масова информация за потребители за един наемател. При подадени userIds връща информация за показване от User / SSOUser.
Използва се от уиджета за коментари за обогатяване на потребители, които току-що са се появили чрез събитие за присъствие.
Няма контекст на страница: поверителността се прилага еднакво (частните профили са маскирани).

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| ids | String | Да |  |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---