## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| state | float64 | Ne |  |
| skip | float64 | Ne |  |
| limit | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Primjer

[inline-code-attrs-start title = 'getTickets Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-789", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
[inline-code-end]

---