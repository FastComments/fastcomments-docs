req
tenantId
urlId
userIdWS

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| userIdWS | string | Sim |  |
| startTime | int64_t | Sim |  |
| endTime | int64_t | Sim |  |

## Resposta

Retorna: [`GetEventLog_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLog_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEventLog'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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