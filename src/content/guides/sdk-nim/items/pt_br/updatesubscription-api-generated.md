## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Não |  |
| userId | string = "" | Não |  |

## Resposta

Retorna: [`Option[UpdateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_subscription_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let subscriptionData = UpdateAPIUserSubscriptionData(
  planId = "premium-plan",
  isActive = true,
  expiresAt = "2025-01-01",
)

let (responseOpt, httpResponse) = client.updateSubscription(
  tenantId = "my-tenant-123",
  id = "sub-456",
  updateAPIUserSubscriptionData = subscriptionData,
  userId = "user-789",
)

if responseOpt.isSome:
  let subscriptionResult = responseOpt.get()
[inline-code-end]