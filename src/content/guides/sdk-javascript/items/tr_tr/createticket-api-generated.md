## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| userId | string | Evet |  |
| createTicketBody | CreateTicketBody | Evet |  |

## Yanıt

Döndürür: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Örnek

[inline-code-attrs-start title = 'createTicket Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string isteğe bağlıdır ve atlanmıştır
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Yanıttan isteğe bağlı bir alanın kullanım örneği
// console.log(response.ticket?.id);
[inline-code-end]