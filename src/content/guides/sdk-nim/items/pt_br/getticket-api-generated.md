## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| userId | string = "" | Não |  |

## Resposta

Retorna: [`Option[GetTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getTicket'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketOpt, httpResp) = client.getTicket(tenantId = "my-tenant-123", id = "ticket-456", userId = "")
if ticketOpt.isSome:
  let ticket = ticketOpt.get()
  discard ticket
[inline-code-end]