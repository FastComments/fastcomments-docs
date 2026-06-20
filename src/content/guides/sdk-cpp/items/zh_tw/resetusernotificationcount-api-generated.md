## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## 範例

[inline-code-attrs-start title = 'resetUserNotificationCount 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotificationCount(tenantId, sso)
    .then([](std::shared_ptr<ResetUserNotificationsResponse> resp){
        if(!resp) resp = std::make_shared<ResetUserNotificationsResponse>();
    })
    .wait();
[inline-code-end]

---