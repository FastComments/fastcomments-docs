## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| userId | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetSubscriptionsAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_subscriptions_api_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getSubscriptions'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSubscriptions(tenantId = "my-tenant-123", userId = "")
if response.isSome:
  let subscriptions = response.get()
  discard subscriptions
[inline-code-end]

---