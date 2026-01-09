## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |

## Réponse

Renvoie : [`Option[GetSubscriptionsAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_subscriptions_api_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSubscriptions'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSubscriptions(tenantId = "my-tenant-123", userId = "")
if response.isSome:
  let subscriptions = response.get()
  discard subscriptions
[inline-code-end]

---