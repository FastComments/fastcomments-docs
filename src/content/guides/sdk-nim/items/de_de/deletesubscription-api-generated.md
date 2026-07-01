---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| userId | string = "" | Nein |  |

## Antwort

Rückgabe: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteSubscription Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.deleteSubscription(
  tenantId = "my-tenant-123",
  id = "sub-789",
  userId = ""
)

if maybeResp.isSome:
  let apiResult = maybeResp.get()
  # Verwenden Sie apiResult nach Bedarf
[inline-code-end]

---