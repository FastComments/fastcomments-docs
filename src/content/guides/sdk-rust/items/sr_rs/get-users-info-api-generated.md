Пакет информација о корисницима за tenant. На основу userIds, враћа информације за приказ из User / SSOUser.
Користи га видгет коментара да обогати кориснике који су управо појавили преко presence event-а.
Нема контекста странице: приватност се примењује униформно (приватни профили су маскирани).

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| ids | String | Да |  |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Пример

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