req
tenantId
urlId
userIdWS

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | Yes |  |
| startTime | int64_t | Yes |  |
| endTime | int64_t | No |  |

## Odgovor

Vraća: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Primjer

[inline-code-attrs-start title = 'getGlobalEventLog Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getGlobalEventLog(
    U("my-tenant-123"),
    U("article-456"),
    U("user@example.com"),
    1622505600,
    boost::optional<int64_t>(1625097600)
).then([](std::shared_ptr<GetEventLogResponse> resp) {
    (void)resp;
});
[inline-code-end]