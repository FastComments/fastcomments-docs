## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| options | const GetNotificationCountOptions& | Так |  |

## Відповідь

Повертає: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
GetNotificationCountOptions options;
options.filter = boost::optional<utility::string_t>(U("unread"));
options.maxCount = boost::optional<int>(10);
api->getNotificationCount(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetNotificationCountResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception& ex) {
        }
    });
[inline-code-end]