req
tenantId
urlId
userIdWS

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| userIdWS | string | Да |  |
| startTime | int64_t | Да |  |
| endTime | int64_t | Не |  |

## Одговор

Враћа: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Пример

[inline-code-attrs-start title = 'Primer getEventLog'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto startTime = int64_t(1622505600);
boost::optional<int64_t> endTime = int64_t(1622592000);
api->getEventLog(U("my-tenant-123"), U("article-456"), U("user@example.com"), startTime, endTime)
    .then([](std::shared_ptr<GetEventLogResponse> response){
        auto copy = std::make_shared<GetEventLogResponse>(*response);
        (void)copy;
    });
[inline-code-end]

---