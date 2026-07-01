## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| options | GetTicketsOptions | No |  |

## Risposta

Restituisce: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketsOpt, httpResp) = client.getTickets(tenantId = "my-tenant-123", options = GetTicketsOptions())
if ticketsOpt.isSome:
  let tickets = ticketsOpt.get()
  # usa i ticket secondo necessità
[inline-code-end]

---