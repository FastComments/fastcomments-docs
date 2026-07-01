## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCountResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio getUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");
api->getUserNotificationCount(tenantId, sso).then([](pplx::task<std::shared_ptr<GetUserNotificationCountResponse>> t){
    try{
        auto resp = t.get();
        // usa resp secondo necessità
    }catch(const std::exception&){
        // gestisci l'errore
    }
});
[inline-code-end]

---