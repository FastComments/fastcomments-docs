## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| userId | string | Ne |  |

## Odgovor

Vraća: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteSubscription(tenantId = "my-tenant-123", id = "sub-98765", userId = "user-456")
if response.isSome:
  let deleteResp = response.get()
  echo "Delete subscription response received"
else:
  echo "No subscription response"
[inline-code-end]

---