## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| externalId | string | Não |  |
| eventType | string | Não |  |
| domain | string | Não |  |
| attemptCountGT | float64 | Não |  |

## Resposta

Retorna: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPendingWebhookEventCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEventCount(
  tenantId = "my-tenant-123",
  commentId = "cmt-456abc",
  externalId = "ext-7890",
  eventType = "comment_created",
  domain = "news.example.com",
  attemptCountGT = 2.0
)

if response.isSome:
  let pending = response.get()
  echo pending
else:
  echo "No pending webhook event count returned; HTTP status: ", httpResponse.status
[inline-code-end]

---