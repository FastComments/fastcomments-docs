## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på deleteEmailTemplate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-001");
boost::optional<utility::string_t> auditReason = boost::none;
api->deleteEmailTemplate(tenantId, templateId).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        if(!resp) {
            auto fallback = std::make_shared<APIEmptyResponse>();
        }
    } catch(const std::exception&) {
    }
});
[inline-code-end]