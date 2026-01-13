## 参数

| Name | Type | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetUserNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCount_200_response.h)

## 示例

[inline-code-attrs-start title = 'getUserNotificationCount 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
std::shared_ptr<GetUserNotificationCount_200_response> result;
api->getUserNotificationCount(tenantId, sso)
.then([&result](pplx::task<std::shared_ptr<GetUserNotificationCount_200_response>> t) {
    try {
        result = t.get();
        if (!result) result = std::make_shared<GetUserNotificationCount_200_response>();
    } catch (...) {
        result = std::make_shared<GetUserNotificationCount_200_response>();
    }
}).wait();
[inline-code-end]

---