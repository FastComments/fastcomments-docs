## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| domain | string | Так |  |

## Відповідь

Повертає: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("example.com");
boost::optional<utility::string_t> env = boost::optional<utility::string_t>(U("production"));
api->getDomainConfig(tenantId, domain).then([=](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetDomainConfig_200_response>();
        std::cout << "Domain config received for " << utility::conversions::to_utf8string(domain) << '\n';
    } catch (const std::exception &ex) {
        std::cerr << "Failed to get domain config: " << ex.what() << '\n';
    }
});
[inline-code-end]

---