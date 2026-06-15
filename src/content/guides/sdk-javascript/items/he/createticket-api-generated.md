## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | כן |  |
| createTicketBody | CreateTicketBody | כן |  |

## תגובה

מחזיר: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp';
const userId: string = 'moderator_jane';
const createTicketBody: CreateTicketBody = {
  subject: 'Mass spam reports on article 789',
  description: 'Multiple identical spam comments posted under article 789. Needs moderation and bulk removal.',
  priority: 'high',
  contactEmail: 'jane@acme-corp.com',
  metadata: { articleId: '789', reportedCount: 12 } // דוגמה למטא-נתונים אופציונליים
};
const ticket: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]