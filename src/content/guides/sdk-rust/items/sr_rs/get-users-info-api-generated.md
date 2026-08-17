Груписане информације о корисницима за закупца. Дати userIds, враћају се подаци за приказ из User / SSOUser.  
Користи се у виџету за коментаре да обогати кориснике који су управо појавили путем догађаја присутности.  
Без контекста странице: приватност се примењује уједначено (приватни профили су маскирани).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## Одговор

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Пример

[inline-code-attrs-start title = 'Primer get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]