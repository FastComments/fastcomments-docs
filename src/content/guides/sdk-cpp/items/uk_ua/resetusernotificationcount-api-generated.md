## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад використання resetUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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