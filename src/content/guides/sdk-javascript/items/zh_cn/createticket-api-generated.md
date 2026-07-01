## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| createTicketBody | CreateTicketBody | 是 |  |

## 响应

返回：[`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## 示例

[inline-code-attrs-start title = 'createTicket 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string 是可选的，已省略
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// 示例：使用响应中的可选字段
// console.log(response.ticket?.id);
[inline-code-end]