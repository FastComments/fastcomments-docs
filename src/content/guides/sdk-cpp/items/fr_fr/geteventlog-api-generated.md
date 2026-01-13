req
tenantId
urlId
userIdWS

## Paramètres

| Name | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| userIdWS | string | Oui |  |
| startTime | int64_t | Oui |  |
| endTime | int64_t | Oui |  |

## Réponse

Renvoie: [`GetEventLog_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLog_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEventLog'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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