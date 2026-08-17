## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domain | string | Да |  |

## Одговор

Враћа: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("myblog.example.com");

api->getDomainConfig(tenantId, domain)
    .then([](std::shared_ptr<GetDomainConfigResponse> response) {
        if (!response) return;
        boost::optional<bool> moderationEnabled = response->moderationEnabled;
        boost::optional<std::string> theme = response->theme;
        if (moderationEnabled && *moderationEnabled) {
            // обради омогућену модерацију
        }
        if (theme) {
            // користи вредност теме
        }
    })
    .wait();
[inline-code-end]

---