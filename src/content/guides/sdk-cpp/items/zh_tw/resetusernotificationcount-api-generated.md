## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ResetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotifications_200_response.h)

## 範例

[inline-code-attrs-start title = 'resetUserNotificationCount 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotificationCount(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<ResetUserNotifications_200_response>> task) {
        try {
            auto resp = task.get();
            if (!resp) resp = std::make_shared<ResetUserNotifications_200_response>();
        } catch (const std::exception &e) {
            (void)e;
        }
    });
[inline-code-end]