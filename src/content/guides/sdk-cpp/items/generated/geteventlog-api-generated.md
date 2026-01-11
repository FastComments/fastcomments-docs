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

[inline-code-attrs-start title = 'getEventLog Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("post-789");
utility::string_t userIdWS = U("alice@example.com");
auto now = std::chrono::system_clock::now();
int64_t endTime = std::chrono::duration_cast<std::chrono::milliseconds>(now.time_since_epoch()).count();
int64_t startTime = endTime - int64_t(24 * 60 * 60 * 1000);
boost::optional<std::shared_ptr<GetEventLog_200_response>> optResponse = std::make_shared<GetEventLog_200_response>();
api->getEventLog(tenantId, urlId, userIdWS, startTime, endTime)
    .then([&optResponse](pplx::task<std::shared_ptr<GetEventLog_200_response>> t) {
        try {
            auto resp = t.get();
            optResponse = resp ? resp : std::make_shared<GetEventLog_200_response>();
        } catch (...) {
            optResponse = std::make_shared<GetEventLog_200_response>();
        }
    });
[inline-code-end]
