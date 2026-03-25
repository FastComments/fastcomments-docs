## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | いいえ |  |
| id | string | いいえ |  |
| changeTicketStateBody | ChangeTicketStateBody | いいえ |  |

## レスポンス

戻り値: [`Option[ChangeTicketState_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state200response.nim)

## 例

[inline-code-attrs-start title = 'changeTicketState の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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