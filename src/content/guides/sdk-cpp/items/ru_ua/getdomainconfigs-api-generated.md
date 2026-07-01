## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |

## Відповідь

Повертає: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getDomainConfigs'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> optionalTenant = tenantId;
api->getDomainConfigs(optionalTenant.value())
    .then([](std::shared_ptr<GetDomainConfigsResponse> response) {
        auto domains = response->getDomainList();
        for (const auto& d : domains) {
            std::cout << d << std::endl;
        }
    });
[inline-code-end]