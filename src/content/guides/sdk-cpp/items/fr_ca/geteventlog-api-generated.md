req
tenantId
urlId
userIdWS

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| userIdWS | string | Oui |  |
| startTime | int64_t | Oui |  |
| endTime | int64_t | Non |  |

## Réponse

Retourne: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEventLog'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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