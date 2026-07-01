## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## 响应

返回：[`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantManualBadgesResponse.h)

## 示例

[inline-code-attrs-start title = 'getManualBadges 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::make_optional(U("user@example.com"));

api->getManualBadges(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<GetTenantManualBadgesResponse>> t) {
        try {
            auto response = t.get();
            // 处理响应，例如 response->badgeList
        } catch (const std::exception& ex) {
            // 处理错误
        }
    });
[inline-code-end]