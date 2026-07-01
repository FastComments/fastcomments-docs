## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | const GetNotificationCountOptions& | Sì |  |

## Risposta

Restituisce: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio getNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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