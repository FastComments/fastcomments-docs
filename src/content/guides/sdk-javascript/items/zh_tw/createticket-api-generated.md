## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## 回應

回傳: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## 範例

[inline-code-attrs-start title = 'createTicket 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const userId: string = "user_72b9f4";
const createTicketBody: CreateTicketBody = {
  subject: "Subscription renewal failed for card on file",
  description: "Customer's card was declined by the payment processor during automatic renewal. Transaction ID: txn_9a8b7c. Please review gateway logs and retry.",
  priority: "high", // 示範可選欄位
  contactEmail: "billing@acme-corp.com", // 可選聯絡資訊
  relatedUrl: "https://acme-corp.com/account/billing"
};
const ticketResponse: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]