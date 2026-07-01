## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| sure | string | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'deleteTenant Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
boost::optional<utility::string_t> sure = utility::conversions::to_string_t("true");
api->deleteTenant(tenantId, userId, sure)
   .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
       try{
           auto resp = t.get();
           auto result = std::make_shared<APIEmptyResponse>(*resp);
       }catch(const std::exception&){})
[inline-code-end]

---