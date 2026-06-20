## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| sendEmail | string | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'deleteModerator Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t moderatorId = U("moderator-456");
boost::optional<utility::string_t> sendEmail = boost::optional<utility::string_t>(U("notify@example.com"));
api->deleteModerator(tenantId, moderatorId, sendEmail)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<APIEmptyResponse>();
    } catch (...) {
        throw;
    }
}).wait();
[inline-code-end]

---