---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
boost::optional<utility::string_t> ifMatch = U("W/\"etag-789\"");

api->deleteTenantPackage(tenantId, packageId)
.then([=](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto finalResp = resp ? resp : std::make_shared<APIEmptyResponse>();
    } catch (const std::exception& e) {
        auto errorResp = std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]

---