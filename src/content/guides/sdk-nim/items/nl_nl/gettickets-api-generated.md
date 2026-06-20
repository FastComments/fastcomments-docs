## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| state | float64 | Nee |  |
| skip | float64 | Nee |  |
| limit | float64 | Nee |  |

## Respons

Retourneert: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getTickets Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-789", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
[inline-code-end]

---