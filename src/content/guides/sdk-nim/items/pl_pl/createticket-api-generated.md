## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| userId | string | Nie |  |
| createTicketBody | CreateTicketBody | Nie |  |

## Odpowiedź

Zwraca: [`Option[CreateTicket_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład createTicket'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateTicketBody(
  title = "Unable to post comment",
  description = "HTTP 500 when submitting comment on article 'world/my-latest-report'",
  contactEmail = "jane.doe@example.com",
  tags = @["comments", "backend"],
  urgent = false
)
let (response, httpResponse) = client.createTicket(tenantId = "my-tenant-123", userId = "user-9876", createTicketBody = createBody)
if response.isSome:
  let ticket = response.get()
  echo "Created ticket ID: ", $ticket
[inline-code-end]

---