## 参数

| 名称 | 类型 | 必需 | 说明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| userId | string | 否 |  |

## 响应

返回: [`Option[GetTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket_response.nim)

## 示例

[inline-code-attrs-start title = 'getTicket 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTicket(tenantId = "my-tenant-123", id = "ticket-456", userId = "user-789")
if response.isSome:
  let ticket = response.get()
  echo "Got ticket:", ticket
else:
  echo "No ticket returned; HTTP response:", httpResponse
[inline-code-end]