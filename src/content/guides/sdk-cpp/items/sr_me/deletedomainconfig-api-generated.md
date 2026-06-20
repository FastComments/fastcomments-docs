---
## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domain | string | Да |  |

## Одговор

Враћа: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfigResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример deleteDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("example.com");
boost::optional<utility::string_t> correlationId = boost::optional<utility::string_t>(U("corr-789"));
api->deleteDomainConfig(tenantId, domain).then([correlationId](pplx::task<std::shared_ptr<DeleteDomainConfigResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<DeleteDomainConfigResponse>();
        utility::string_t cid = correlationId ? *correlationId : U("");
        (void)cid;
    } catch(const std::exception &){
        auto err = std::make_shared<DeleteDomainConfigResponse>();
        (void)err;
    }
});
[inline-code-end]

---