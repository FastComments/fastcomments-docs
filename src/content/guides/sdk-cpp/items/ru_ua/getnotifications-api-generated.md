## Параметри

| Назва | Тип | Обовʼязково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| options | const GetNotificationsOptions& | Так |  |

## Відповідь

Повертає: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetNotificationsOptions options;
options.limit = 20;
options.after = U("cursor-123");
api->getNotifications(U("my-tenant-123"), options)
    .then([](std::shared_ptr<GetNotificationsResponse> resp) {
        (void)resp;
    });
[inline-code-end]