req
tenantId
urlId
userIdWS

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| userIdWS | string | Não |  |
| startTime | int64 | Não |  |
| endTime | int64 | Não |  |

## Resposta

Retorna: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-2026-solar-panels",
  userIdWS = "user-456",
  startTime = 1688000000'i64,
  endTime = 1688086400'i64
)
if response.isSome:
  let eventLog = response.get()
  discard eventLog
[inline-code-end]

---