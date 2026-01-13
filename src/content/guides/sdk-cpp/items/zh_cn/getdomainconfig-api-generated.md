## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| domain | string | 是 |  |

## 响应

返回: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## 示例

[inline-code-attrs-start title = 'getDomainConfig 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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