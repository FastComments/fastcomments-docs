## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| id | string | 否 |  |
| changeTicketStateBody | ChangeTicketStateBody | 否 |  |

## 响应

返回：[`Option[ChangeTicketState_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state200response.nim)

## 示例

[inline-code-attrs-start title = 'changeTicketState 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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