req
tenantId
urlId
userIdWS

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Ja |  |
| startTime | int64_t | Ja |  |
| endTime | int64_t | Nee |  |

## Antwoord

Retourneert: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getGlobalEventLog Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
utility::string_t userIdWS = U("user@example.com");
int64_t startTime = 1622505600000LL;
boost::optional<int64_t> endTime = boost::optional<int64_t>(1622592000000LL);
auto task = api->getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime)
    .then([](pplx::task<std::shared_ptr<GetEventLogResponse>> t){
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<GetEventLogResponse>();
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]