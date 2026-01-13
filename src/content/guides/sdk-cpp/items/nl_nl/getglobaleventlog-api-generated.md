req
tenantId
urlId
userIdWS

## Parameters

| Name | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Ja |  |
| startTime | int64_t | Ja |  |
| endTime | int64_t | Ja |  |

## Response

Retourneert: [`GetEventLog_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLog_200_response.h)

## Voorbeeld

[inline-code-attrs-start title = 'getGlobalEventLog Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t defaultUrlId = U("article-456");
boost::optional<utility::string_t> maybeUrlId = boost::optional<utility::string_t>(U("article-456"));
utility::string_t urlId = maybeUrlId ? *maybeUrlId : defaultUrlId;
utility::string_t userIdWS = U("user@example.com");
int64_t startTime = 1622505600000LL;
int64_t endTime = 1622592000000LL;
auto defaultResp = std::make_shared<GetEventLog_200_response>();
api->getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime)
.then([](pplx::task<std::shared_ptr<GetEventLog_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) std::cout << "Received event log with entries\n";
        else std::cout << "No event log returned\n";
    } catch (const std::exception &e) {
        std::cerr << "getGlobalEventLog failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---