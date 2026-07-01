## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| id | string | Não |  |
| changeTicketStateBody | ChangeTicketStateBody | Não |  |

## Resposta

Retorna: [`Option[ChangeTicketStateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo changeTicketState'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.changeTicketState(
  tenantId = "my-tenant-001",
  userId = "user-42",
  id = "ticket-12345",
  changeTicketStateBody = ChangeTicketStateBody(state = "closed")
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]