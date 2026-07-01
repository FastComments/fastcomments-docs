## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| options | const GetNotificationCountOptions& | Tak |  |

## Odpowiedź

Zwraca: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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