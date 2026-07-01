Bulk gebruikersinfo voor een tenant. Gegeven userIds, retourneer weergave‑info van User / SSOUser.  
Gebruikt door de commentaarwidget om gebruikers die net verschenen via een presence‑event te verrijken.  
Geen paginacontext: privacy wordt uniform afgedwongen (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Response

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getUsersInfo Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // process response
    }catch(const std::exception&){
        // handle error
    }
});
[inline-code-end]

---