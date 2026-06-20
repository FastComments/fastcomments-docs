req
tenantId
urlId
userIdWS

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userIdWS | string | 예 |  |
| startTime | int64_t | 예 |  |
| endTime | int64_t | 아니요 |  |

## 응답

반환: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## 예제

[inline-code-attrs-start title = 'getEventLog 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
utility::string_t userIdWS = U("user@example.com");
int64_t startTime = 1654041600000LL;
boost::optional<int64_t> endTime = boost::optional<int64_t>(1656643200000LL);
api->getEventLog(tenantId, urlId, userIdWS, startTime, endTime)
    .then([](pplx::task<std::shared_ptr<GetEventLogResponse>> t){
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<GetEventLogResponse>();
            std::cout << "Event log fetched, pointer: " << result.get() << std::endl;
        } catch (const std::exception &e) {
            std::cerr << "getEventLog error: " << e.what() << std::endl;
        }
    });
[inline-code-end]

---