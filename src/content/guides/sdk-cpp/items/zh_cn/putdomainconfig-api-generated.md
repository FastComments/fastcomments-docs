## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| domainToUpdate | string | 是 |  |
| updateDomainConfigParams | UpdateDomainConfigParams | 是 |  |

## 响应

返回: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutDomainConfigResponse.h)

## 示例

[inline-code-attrs-start title = 'putDomainConfig 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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