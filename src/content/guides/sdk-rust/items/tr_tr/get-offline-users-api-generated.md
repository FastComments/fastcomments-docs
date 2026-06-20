---
Sayfadaki şu anda çevrimiçi olmayan önceki yorumcular. displayName'e göre sıralanmıştır.
/users/online'ı tükettikten sonra 'Üyeler' bölümünü görüntülemek için bunu kullanın.
commenterName üzerinde cursor sayfalama: sunucu, kısmi {tenantId, urlId, commenterName} dizinini afterName'den başlayarak $gt ile ileri doğru dolaşır; $skip maliyeti yok.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| after_name | String | Hayır |  |
| after_user_id | String | Hayır |  |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_offline_users Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---