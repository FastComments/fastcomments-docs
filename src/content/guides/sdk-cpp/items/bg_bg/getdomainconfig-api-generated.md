## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domain | string | Да |  |

## Отговор

Връща: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> domainOpt = U("app.example.com");
if (domainOpt) {
    api->getDomainConfig(tenantId, *domainOpt)
    .then([](pplx::task<std::shared_ptr<GetDomainConfigResponse>> t) {
        try {
            auto resp = t.get();
            auto cfgCopy = std::make_shared<GetDomainConfigResponse>(*resp);
            (void)cfgCopy;
        } catch (const std::exception&) {
        }
    });
}
[inline-code-end]

---