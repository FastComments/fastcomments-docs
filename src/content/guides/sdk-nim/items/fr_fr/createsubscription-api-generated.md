## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Non |  |

## Réponse

Retourne : [`Option[CreateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_subscription_api_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple createSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.createSubscription(
  tenantId = "my-tenant-123",
  createAPIUserSubscriptionData = default(CreateAPIUserSubscriptionData),
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]