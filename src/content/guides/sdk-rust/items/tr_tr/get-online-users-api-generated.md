Şu anda çevrimiçi sayfa izleyicileri: WebSocket oturumu şu anda sayfaya abone olan kişiler.  
anonCount + totalCount değerini döndürür (odadaki aboneler, saymadığımız anonim izleyicileri içerir).

## Parameters

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| after_name | String | Hayır |  |
| after_user_id | String | Hayır |  |

## Response

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Example

[inline-code-attrs-start title = 'get_online_users Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]