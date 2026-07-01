## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| options | const GetNotificationsOptions& | Ja |  |

## Antwort

Rückgabe: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getNotifications Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetNotificationsOptions options;
options.limit = 20;
options.after = U("cursor-123");
api->getNotifications(U("my-tenant-123"), options)
    .then([](std::shared_ptr<GetNotificationsResponse> resp) {
        (void)resp;
    });
[inline-code-end]