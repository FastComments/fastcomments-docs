## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| createTicketBody | CreateTicketBody | No |  |

## Risposta

Restituisce: [`Option[CreateTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di createTicket'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = CreateTicketBody(
  subject = "Comment moderation issue",
  message = "Several abusive comments reported on article, please review and moderate.",
  tags = @["moderation", "abuse", "urgent"],
  url = "https://news.example.com/world/2026-election",
  priority = "high"
)

let (response, httpResponse) = client.createTicket(tenantId = "my-tenant-123", userId = "user-789", createTicketBody = body)

if response.isSome:
  let ticket = response.get()
  echo "Created ticket ID: ", ticket.id
[inline-code-end]

---