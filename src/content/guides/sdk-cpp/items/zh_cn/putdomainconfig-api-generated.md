## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| domainToUpdate | string | 是 |  |
| updateDomainConfigParams | UpdateDomainConfigParams | 是 |  |

## 响应

返回: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## 示例

[inline-code-attrs-start title = 'putDomainConfig 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("comments.myapp.com");
auto params = std::make_shared<UpdateDomainConfigParams>();
params->displayName = utility::string_t(U("My App Comments"));
params->enabled = boost::optional<bool>(true);
params->contactEmail = boost::optional<utility::string_t>(U("admin@myapp.com"));
api->putDomainConfig(tenantId, domainToUpdate, *params)
.then([](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto updated = resp;
        }
    } catch (const std::exception& e) {
    }
});
[inline-code-end]