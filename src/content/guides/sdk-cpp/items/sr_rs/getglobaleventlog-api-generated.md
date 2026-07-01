req
tenantId
urlId
userIdWS

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| userIdWS | string | Da |  |
| startTime | int64_t | Da |  |
| endTime | int64_t | Ne |  |

## Odgovor

Vraća: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Primer

[inline-code-attrs-start title = 'getGlobalEventLog Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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