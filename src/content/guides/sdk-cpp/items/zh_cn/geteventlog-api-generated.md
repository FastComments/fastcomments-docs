req
tenantId
urlId
userIdWS

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 是 |  |
| startTime | int64_t | 是 |  |
| endTime | int64_t | 是 |  |

## 响应

返回: [`GetEventLog_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLog_200_response.h)

## 示例

[inline-code-attrs-start title = 'getEventLog 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
utility::string_t userIdWS = U("user@example.com");
int64_t startTime = 1672531200LL;
boost::optional<int64_t> endTimeOpt = 1672617600LL;
api->getEventLog(tenantId, urlId, userIdWS, startTime, *endTimeOpt)
    .then([](pplx::task<std::shared_ptr<GetEventLog_200_response>> t){
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<GetEventLog_200_response>();
        } catch (const std::exception&) {}
    });
[inline-code-end]