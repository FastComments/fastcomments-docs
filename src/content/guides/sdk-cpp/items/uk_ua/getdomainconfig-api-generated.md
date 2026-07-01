## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|------------|------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Відповідь

Повертає: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("myblog.example.com");

api->getDomainConfig(tenantId, domain)
    .then([](std::shared_ptr<GetDomainConfigResponse> response) {
        if (!response) return;
        boost::optional<bool> moderationEnabled = response->moderationEnabled;
        boost::optional<std::string> theme = response->theme;
        if (moderationEnabled && *moderationEnabled) {
            // обробляти ввімкнену модерацію
        }
        if (theme) {
            // використати значення теми
        }
    })
    .wait();
[inline-code-end]