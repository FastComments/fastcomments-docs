## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetTicketsOptions | Ne |  |

## Odgovor

Returns: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketsOpt, httpResp) = client.getTickets(tenantId = "my-tenant-123", options = GetTicketsOptions())
if ticketsOpt.isSome:
  let tickets = ticketsOpt.get()
  # use tickets as needed
[inline-code-end]