## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| sso | string | Нет |  |

## Response

Возвращает: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример resetUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto resetTask = api->resetUserNotificationCount(
    U("my-tenant-123"),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](std::shared_ptr<ResetUserNotificationsResponse> resp){
});
[inline-code-end]

---