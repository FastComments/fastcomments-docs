## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | Yes |  |
| startTime | int64_t | Yes |  |
| endTime | int64_t | Yes |  |

## Response

Returns: [`GetEventLog_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLog_200_response.h)

## Example

[inline-code-attrs-start title = 'getGlobalEventLog Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t userIdWS = U("user@example.com");
int64_t startTime = 1700000000000LL;
int64_t endTime = 1700003600000LL;
boost::optional<utility::string_t> optionalFilter = boost::none;
auto task = api->getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime)
.then([](pplx::task<std::shared_ptr<GetEventLog_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto result = resp;
            (void)result;
        }
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<GetEventLog_200_response>();
        (void)fallback;
    }
});
[inline-code-end]
