## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## 回應

返回：[`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## 範例

[inline-code-attrs-start title = 'createTicket 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string 是可選的，已省略
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// 範例：使用回應中的可選字段
// console.log(response.ticket?.id);
[inline-code-end]