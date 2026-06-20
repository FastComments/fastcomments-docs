## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| addDomainConfigParams | AddDomainConfigParams | Да |  |

## Отговор

Връща: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddDomainConfigResponse.h)

## Пример

[inline-code-attrs-start title = 'addDomainConfig Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
AddDomainConfigParams params;
params.domain = U("comments.example.com");
params.ownerEmail = U("admin@example.com");
params.enforceHttps = boost::optional<bool>(true);
params.note = boost::optional<utility::string_t>(U("Primary comments host for example.com"));
api->addDomainConfig(tenantId, params)
.then([](std::shared_ptr<AddDomainConfigResponse> resp){
    if(!resp) throw std::runtime_error("addDomainConfig returned null");
    return std::make_shared<AddDomainConfigResponse>(*resp);
})
.then([](std::shared_ptr<AddDomainConfigResponse> finalResp){
    (void)finalResp;
});
[inline-code-end]

---