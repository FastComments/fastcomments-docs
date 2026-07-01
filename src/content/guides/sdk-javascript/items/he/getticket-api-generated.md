## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## תשובה

מחזירה: [`GetTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-inc";
const ticketId: string = "ticket-3421";
const userId: string = "alice.smith";

const ticketWithUser: GetTicketResponse1 = await getTicket(tenantId, ticketId, userId);
const ticketWithoutUser: GetTicketResponse1 = await getTicket(tenantId, ticketId);
[inline-code-end]