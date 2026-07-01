## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'deleteEmailTemplate Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto templateId = utility::conversions::to_string_t("welcome-email-template");
boost::optional<utility::string_t> optTenantId = tenantId;
boost::optional<utility::string_t> optTemplateId = templateId;
api->deleteEmailTemplate(optTenantId.value(), optTemplateId.value())
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try { t.get(); }
        catch (const std::exception&) {}
    });
[inline-code-end]