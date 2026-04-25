---
## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| createTicketBody | CreateTicketBody | 是 |  |

## 回應

回傳: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## 範例

[inline-code-attrs-start title = 'createTicket 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '7f3e9b1a-1c2d-4a5b-b6c7-d8e9f0123456';
const userId: string = 'd290f1ee-6c54-4b01-90e6-d701748f0851';
const createTicketBody: CreateTicketBody = {
  subject: 'Unable to post comments on product update',
  message: 'Submitting a comment returns a 504 timeout after ~10s. Reproducible in Chrome and Firefox.',
  priority: 'high', // 可選欄位（已包含）
  contactEmail: 'jane.doe@acme-corp.com',
  ccEmails: ['eng-oncall@acme-corp.com'], // 可選欄位（已包含）
  metadata: { page: '/blog/product-update', browser: 'Chrome 112' } // 可選
};
const response: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]

---