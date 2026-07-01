## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string = "" | No |  |

## Απάντηση

Επιστρέφει: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.deleteSubscription(
  tenantId = "my-tenant-123",
  id = "sub-789",
  userId = ""
)

if maybeResp.isSome:
  let apiResult = maybeResp.get()
  # χρησιμοποιήστε το apiResult όπως χρειάζεται
[inline-code-end]

---