req
tenantId
urlId
userIdWS

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| userIdWS | string | Да |  |
| startTime | int64_t | Да |  |
| endTime | int64_t | Не |  |

## Отговор

Връща: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getEventLog'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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