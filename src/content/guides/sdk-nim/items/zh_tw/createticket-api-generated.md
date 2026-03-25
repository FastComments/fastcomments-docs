## 參數

| 名稱 | 類型 | 必須 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| createTicketBody | CreateTicketBody | 否 |  |

## 回應

回傳: [`Option[CreateTicket_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket200response.nim)

## 範例

[inline-code-attrs-start title = 'createTicket 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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