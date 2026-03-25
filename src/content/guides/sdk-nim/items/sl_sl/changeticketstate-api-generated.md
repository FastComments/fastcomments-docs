## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| id | string | Ne |  |
| changeTicketStateBody | ChangeTicketStateBody | Ne |  |

## Odgovor

Vrne: [`Option[ChangeTicketState_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer changeTicketState'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.changeTicketState(
  tenantId = "my-tenant-123",
  userId = "user-456",
  id = "ticket-789",
  changeTicketStateBody = ChangeTicketStateBody(
    state = "closed",
    message = "Issue resolved by support",
    notify = true,
    tags = @["support", "resolved"]
  )
)
if response.isSome:
  let result = response.get()
  echo "Changed ticket:", result.state, " (id: ", result.id, ")"
[inline-code-end]

---