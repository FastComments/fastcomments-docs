## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | No |  |

## Risposta

Restituisce: [`Option[CreateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_subscription_api_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio createSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.createSubscription(
  tenantId = "my-tenant-123",
  createAPIUserSubscriptionData = default(CreateAPIUserSubscriptionData),
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]

---