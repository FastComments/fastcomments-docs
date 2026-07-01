---  
Kiracı için toplu kullanıcı bilgisi. Verilen userIds ile User / SSOUser'dan görüntüleme bilgisi döndürülür.  
Yorum widget'ı tarafından, varlık olayıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.  
Sayfa bağlamı yok: gizlilik tutarlı bir şekilde uygulanır (özel profiller maskeleştirilir).

## Parameters

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_users_info Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
let params = GetUsersInfoParams {  
    tenant_id: "acme-corp-tenant".to_string(),  
    ids: "user-1,user-2".to_string(),  
};  
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;  
[inline-code-end]  

---