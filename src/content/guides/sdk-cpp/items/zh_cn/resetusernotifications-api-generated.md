## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| afterCreatedAt | int64_t | 否 |  |
| unreadOnly | bool | 否 |  |
| dmOnly | bool | 否 |  |
| noDm | bool | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`ResetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotifications_200_response.h)

## 示例

[inline-code-attrs-start title = 'resetUserNotifications 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(U("notif-987654321"));
boost::optional<int64_t> afterCreatedAt = boost::optional<int64_t>(1625097600000LL);
boost::optional<bool> unreadOnly = boost::optional<bool>(true);
boost::optional<bool> dmOnly = boost::optional<bool>(false);
boost::optional<bool> noDm;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, noDm, sso)
    .then([](pplx::task<std::shared_ptr<ResetUserNotifications_200_response>> t)
    {
        try
        {
            auto resp = t.get();
            auto respCopy = std::make_shared<ResetUserNotifications_200_response>(*resp);
            return respCopy;
        }
        catch (...)
        {
            return std::shared_ptr<ResetUserNotifications_200_response>();
        }
    });
[inline-code-end]

---