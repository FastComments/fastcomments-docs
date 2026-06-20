## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| userId | string | Nee |  |

## Antwoord

Retourneert: [`Option[GetTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getTicket Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTicket(tenantId = "my-tenant-123", id = "ticket-456", userId = "user-789")
if response.isSome:
  let ticket = response.get()
  echo "Got ticket:", ticket
else:
  echo "No ticket returned; HTTP response:", httpResponse
[inline-code-end]

---