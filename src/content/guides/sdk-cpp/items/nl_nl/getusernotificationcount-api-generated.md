## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCountResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");
api->getUserNotificationCount(tenantId, sso).then([](pplx::task<std::shared_ptr<GetUserNotificationCountResponse>> t){
    try{
        auto resp = t.get();
        // gebruik resp indien nodig
    }catch(const std::exception&){
        // verwerk fout
    }
});
[inline-code-end]