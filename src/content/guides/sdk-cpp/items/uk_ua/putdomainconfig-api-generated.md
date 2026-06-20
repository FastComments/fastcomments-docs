## Параметри

| Ім'я | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| domainToUpdate | string | Так |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Так |  |

## Відповідь

Повертає: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutDomainConfigResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад putDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
boost::optional<utility::string_t> contactEmail = U("admin@example.com");
boost::optional<bool> enforceHttps = true;
UpdateDomainConfigParams updateParams;
updateParams.contactEmail = contactEmail;
updateParams.enforceHttps = enforceHttps;
api->putDomainConfig(tenantId, domainToUpdate, updateParams)
.then([](pplx::task<std::shared_ptr<PutDomainConfigResponse>> t){
    try {
        auto resp = t.get();
        return resp ? resp : std::make_shared<PutDomainConfigResponse>();
    } catch(...) {
        return std::make_shared<PutDomainConfigResponse>();
    }
});
[inline-code-end]

---