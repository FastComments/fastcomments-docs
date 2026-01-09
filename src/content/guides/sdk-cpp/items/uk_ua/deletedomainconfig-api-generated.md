## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| domain | string | Так |  |

## Відповідь

Повертає: [`DeleteDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfig_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("comments.example.com");
boost::optional<utility::string_t> ifMatch = boost::optional<utility::string_t>(U("W/\"abc123\""));
api->deleteDomainConfig(tenantId, domain)
.then([](pplx::task<std::shared_ptr<DeleteDomainConfig_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto resultCopy = std::make_shared<DeleteDomainConfig_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---