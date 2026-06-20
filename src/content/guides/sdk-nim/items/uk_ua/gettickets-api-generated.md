## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Ні |  |
| state | float64 | Ні |  |
| skip | float64 | Ні |  |
| limit | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-789", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
[inline-code-end]

---