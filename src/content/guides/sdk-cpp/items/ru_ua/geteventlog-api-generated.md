req
tenantId
urlId
userIdWS

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | Yes |  |
| startTime | int64_t | Yes |  |
| endTime | int64_t | No |  |

## Відповідь

Повертає: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Приклад

[inline-code-attrs-start title = 'getEventLog Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto startTime = int64_t(1622505600);
boost::optional<int64_t> endTime = int64_t(1622592000);
api->getEventLog(U("my-tenant-123"), U("article-456"), U("user@example.com"), startTime, endTime)
    .then([](std::shared_ptr<GetEventLogResponse> response){
        auto copy = std::make_shared<GetEventLogResponse>(*response);
        (void)copy;
    });
[inline-code-end]