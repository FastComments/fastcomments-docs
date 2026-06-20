---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCountResponse.h)

## 示例

[inline-code-attrs-start title = 'getUserNotificationCount 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto fallback = std::make_shared<GetUserNotificationCountResponse>();
api->getUserNotificationCount(tenantId, sso)
.then([fallback](std::shared_ptr<GetUserNotificationCountResponse> resp) {
    auto result = resp ? resp : fallback;
    std::cout << "Received user notification count response (ptr=" << (result.get() != nullptr) << ")\n";
})
.wait();
[inline-code-end]

---