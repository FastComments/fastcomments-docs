## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domain | string | Ja |  |

## Respons

Retourneert: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getDomainConfig Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("myblog.example.com");

api->getDomainConfig(tenantId, domain)
    .then([](std::shared_ptr<GetDomainConfigResponse> response) {
        if (!response) return;
        boost::optional<bool> moderationEnabled = response->moderationEnabled;
        boost::optional<std::string> theme = response->theme;
        if (moderationEnabled && *moderationEnabled) {
            // verwerk ingeschakelde moderatie
        }
        if (theme) {
            // gebruik themawaarde
        }
    })
    .wait();
[inline-code-end]