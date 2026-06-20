---
Bir kiracı için toplu kullanıcı bilgisi. Given userIds, User / SSOUser'dan görüntüleme bilgilerini döndürür.
Yorum bileşeni, presence event ile yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yok: gizlilik tutarlı olarak uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| ids | String | Evet |  |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_users_info Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]