## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| notificationId | string | כן |  |
| newStatus | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatusResponse.h)

## דוגמה

[inline-code-attrs-start title = 'updateUserNotificationStatus דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->updateUserNotificationStatus(
    U("my-tenant-123"),
    U("notif-456"),
    U("read"),
    boost::optional<utility::string_t>(U("sso-token-abc"))
).then([](std::shared_ptr<UpdateUserNotificationStatusResponse> resp) {
}).wait();
[inline-code-end]