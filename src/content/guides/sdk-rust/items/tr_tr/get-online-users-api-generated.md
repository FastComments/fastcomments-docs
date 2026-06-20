---
Bir sayfanın şu anda çevrimiçi olan izleyicileri: websocket oturumu şu anda sayfaya abone olan kişiler.
anonCount + totalCount döndürür (oda genelindeki aboneler, saymadığımız anonim görüntüleyiciler dahil).

## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| after_name | String | Hayır |  |
| after_user_id | String | Hayır |  |

## Yanıt

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_online_users Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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