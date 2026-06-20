## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| fromName | string | Ja |  |

## Antwoord

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'sendInvite Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
utility::string_t fromName = U("Acme Support");
boost::optional<utility::string_t> note = boost::optional<utility::string_t>(U("Invitation to join comments"));
api->sendInvite(tenantId, id, fromName)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try {
            auto resp = t.get();
            auto finalResp = resp ? resp : std::make_shared<APIEmptyResponse>();
            (void)finalResp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---