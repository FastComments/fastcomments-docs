## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio getUserBadge'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-456");
api->getUserBadge(tenantId, userId).then([](pplx::task<std::shared_ptr<APIGetUserBadgeResponse>> t){
    try{
        auto resp = t.get();
        boost::optional<std::string> badgeUrl = resp->badge_url ? boost::optional<std::string>(*resp->badge_url) : boost::none;
    }catch(const std::exception&){
    }
});
[inline-code-end]

---