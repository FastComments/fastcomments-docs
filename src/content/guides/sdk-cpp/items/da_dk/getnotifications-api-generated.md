## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetNotificationsOptions& | Yes |  |

## Svar

Returnerer: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getNotifications Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetNotificationsOptions options;
options.limit = 20;
options.after = U("cursor-123");
api->getNotifications(U("my-tenant-123"), options)
    .then([](std::shared_ptr<GetNotificationsResponse> resp) {
        (void)resp;
    });
[inline-code-end]