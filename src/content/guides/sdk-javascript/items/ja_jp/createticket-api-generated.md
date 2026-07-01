## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## レスポンス

戻り値: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## 例

[inline-code-attrs-start title = 'createTicket 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string はオプションで、省略されています
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// 応答からオプションフィールドを使用する例
// console.log(response.ticket?.id);
[inline-code-end]