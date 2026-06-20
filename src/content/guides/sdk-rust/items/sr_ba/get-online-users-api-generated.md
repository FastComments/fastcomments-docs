Trenutno online posjetioci stranice: osobe čija je WebSocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici unutar sobe, uključujući anonimne posjetioce koje ne nabrajamo).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| after_name | String | Ne |  |
| after_user_id | String | Ne |  |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_online_users Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_online_users() -> Result<PageUsersOnlineResponse, Error> {
    let params: GetOnlineUsersParams = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/article-2026".to_string(),
        after_name: Some("jane.doe".to_string()),
        after_user_id: Some("user_98765".to_string()),
    };
    let response: PageUsersOnlineResponse = get_online_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---