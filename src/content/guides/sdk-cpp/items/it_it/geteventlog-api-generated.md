req
tenantId
urlId
userIdWS

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| userIdWS | string | Sì |  |
| startTime | int64_t | Sì |  |
| endTime | int64_t | No |  |

## Risposta

Restituisce: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di getEventLog'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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