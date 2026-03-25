## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| userId | string | Evet |  |
| createTicketBody | CreateTicketBody | Evet |  |

## Response

Döndürür: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createTicket Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const userId: string = "user_72b9f4";
const createTicketBody: CreateTicketBody = {
  subject: "Subscription renewal failed for card on file",
  description: "Customer's card was declined by the payment processor during automatic renewal. Transaction ID: txn_9a8b7c. Please review gateway logs and retry.",
  priority: "high", // isteğe bağlı alan örneği
  contactEmail: "billing@acme-corp.com", // isteğe bağlı iletişim bilgisi
  relatedUrl: "https://acme-corp.com/account/billing"
};
const ticketResponse: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]