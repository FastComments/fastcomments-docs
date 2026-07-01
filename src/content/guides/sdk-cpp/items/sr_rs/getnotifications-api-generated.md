## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| options | const GetNotificationsOptions& | Да |  |

## Одговор

Враћа: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## Пример

[inline-code-attrs-start title = 'Primer getNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetNotificationsOptions options;
options.limit = 20;
options.after = U("cursor-123");
api->getNotifications(U("my-tenant-123"), options)
    .then([](std::shared_ptr<GetNotificationsResponse> resp) {
        (void)resp;
    });
[inline-code-end]