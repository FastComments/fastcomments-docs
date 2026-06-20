---
## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| addDomainConfigParams | AddDomainConfigParams | はい |  |

## レスポンス

返却値: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddDomainConfigResponse.h)

## 例

[inline-code-attrs-start title = 'addDomainConfig の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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