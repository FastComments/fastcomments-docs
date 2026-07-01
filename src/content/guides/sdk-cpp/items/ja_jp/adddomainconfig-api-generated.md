## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| addDomainConfigParams | AddDomainConfigParams | はい |  |

## レスポンス

返却: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddDomainConfigResponse.h)

## 例

[inline-code-attrs-start title = 'addDomainConfig の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
AddDomainConfigParams params;
params.domain = U("example.com");
params.adminEmail = U("admin@example.com");
params.notes = boost::optional<utility::string_t>(U("Primary domain"));
api->addDomainConfig(tenantId, params).then([](pplx::task<std::shared_ptr<AddDomainConfigResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]

---