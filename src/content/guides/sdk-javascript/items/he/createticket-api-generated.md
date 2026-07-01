## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | כן |  |
| createTicketBody | CreateTicketBody | כן |  |

## תגובה

מחזיר: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'createTicket דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string הוא אופציונלי ומושמט
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// דוגמה לשימוש בשדה אופציונלי מהתגובה
// console.log(response.ticket?.id);
[inline-code-end]