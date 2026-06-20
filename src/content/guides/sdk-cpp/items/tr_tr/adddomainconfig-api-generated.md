## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| addDomainConfigParams | AddDomainConfigParams | Evet |  |

## Yanıt

Döndürür: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddDomainConfigResponse.h)

## Örnek

[inline-code-attrs-start title = 'addDomainConfig Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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