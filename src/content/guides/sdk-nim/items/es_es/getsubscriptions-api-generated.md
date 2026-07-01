## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string = "" | No |  |

## Respuesta

Devuelve: [`Option[GetSubscriptionsAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_subscriptions_api_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getSubscriptions'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (subscriptionsOpt, httpResp) = client.getSubscriptions(tenantId = "my-tenant-123", userId = "user-456")
if subscriptionsOpt.isSome:
  let subscriptions = subscriptionsOpt.get()
  echo subscriptions
[inline-code-end]