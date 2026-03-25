## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Да |  |
| createTicketBody | CreateTicketBody | Да |  |

## Отговор

Връща: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const userId: string = "user_72b9f4";
const createTicketBody: CreateTicketBody = {
  subject: "Subscription renewal failed for card on file",
  description: "Customer's card was declined by the payment processor during automatic renewal. Transaction ID: txn_9a8b7c. Please review gateway logs and retry.",
  priority: "high", // пример за незадължително поле
  contactEmail: "billing@acme-corp.com", // незадължителна контактна информация
  relatedUrl: "https://acme-corp.com/account/billing"
};
const ticketResponse: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]

---