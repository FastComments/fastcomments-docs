Sayfada daha önce yorum yapmış, şu anda çevrimiçi OLMAYAN yorumcular. displayName'e göre sıralanır.
"Üyeler" bölümünü oluşturmak için /users/online kaynağını tükettikten sonra bunu kullanın.
commenterName üzerinde imleç tabanlı sayfalama: sunucu, {tenantId, urlId, commenterName} kısmi indeksini
afterName'den $gt ile ileriye doğru tarar, $skip maliyeti yoktur.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
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
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]