## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Zwraca: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("myblog.example.com");

api->getDomainConfig(tenantId, domain)
    .then([](std::shared_ptr<GetDomainConfigResponse> response) {
        if (!response) return;
        boost::optional<bool> moderationEnabled = response->moderationEnabled;
        boost::optional<std::string> theme = response->theme;
        if (moderationEnabled && *moderationEnabled) {
            // handle enabled moderation
        }
        if (theme) {
            // use theme value
        }
    })
    .wait();
[inline-code-end]