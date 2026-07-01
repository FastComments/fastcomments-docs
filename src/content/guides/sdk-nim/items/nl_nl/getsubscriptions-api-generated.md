## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| userId | string = "" | Nee |  |

## Respons

Retourneert: [`Option[GetSubscriptionsAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_subscriptions_api_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getSubscriptions Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (subscriptionsOpt, httpResp) = client.getSubscriptions(tenantId = "my-tenant-123", userId = "user-456")
if subscriptionsOpt.isSome:
  let subscriptions = subscriptionsOpt.get()
  echo subscriptions
[inline-code-end]