req
tenantId
urlId
userIdWS

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|----------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Ja |  |
| startTime | int64_t | Ja |  |
| endTime | int64_t | Nein |  |

## Antwort

Rückgabe: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getGlobalEventLog Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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