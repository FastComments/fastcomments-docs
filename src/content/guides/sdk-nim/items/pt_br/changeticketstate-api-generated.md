## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| id | string | Não |  |
| changeTicketStateBody | ChangeTicketStateBody | Não |  |

## Resposta

Retorna: [`Option[ChangeTicketState_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de changeTicketState'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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