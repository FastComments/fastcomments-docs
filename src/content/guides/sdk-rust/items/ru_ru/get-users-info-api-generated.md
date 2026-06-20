Массовая информация о пользователях для арендатора. По заданным userIds возвращает отображаемую информацию из User / SSOUser.
Используется виджетом комментариев для обогащения пользователей, которые только что появились через событие присутствия.
Нет контекста страницы: конфиденциальность соблюдается одинаково (закрытые профили маскируются).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| ids | String | Да |  |

## Response

Возвращает: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'Пример get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---