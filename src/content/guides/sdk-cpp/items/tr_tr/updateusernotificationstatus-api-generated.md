## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|----------|
| tenantId | string | Evet |  |
| notificationId | string | Evet |  |
| newStatus | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatusResponse.h)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationStatus Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->updateUserNotificationStatus(
    U("my-tenant-123"),
    U("notif-456"),
    U("read"),
    boost::optional<utility::string_t>(U("sso-token-abc"))
).then([](std::shared_ptr<UpdateUserNotificationStatusResponse> resp) {
}).wait();
[inline-code-end]