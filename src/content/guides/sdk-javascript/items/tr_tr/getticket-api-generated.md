## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicket200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getTicket Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const ticketId: string = 'TCKT-20250615-42';
const userId: string = 'user_84b2';

const ticketWithUser: GetTicket200Response = await getTicket(tenantId, ticketId, userId);
const ticketWithoutUser: GetTicket200Response = await getTicket(tenantId, ticketId);

console.log(ticketWithUser.id, ticketWithoutUser.id);
[inline-code-end]

---