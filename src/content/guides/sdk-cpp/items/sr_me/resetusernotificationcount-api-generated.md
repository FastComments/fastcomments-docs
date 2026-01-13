## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| sso | string | Не |  |

## Одговор

Враћа: [`ResetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotifications_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример resetUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---