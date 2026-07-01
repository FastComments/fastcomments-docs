## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| id | string | No |  |
| changeTicketStateBody | ChangeTicketStateBody | No |  |

## Odgovor

Vraća: [`Option[ChangeTicketStateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state_response.nim)

## Primjer

[inline-code-attrs-start title = 'changeTicketState Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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