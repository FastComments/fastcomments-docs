## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | כן |  |
| createTicketBody | CreateTicketBody | כן |  |

## תגובה

מחזיר: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-company-001';
const userId: string = 'u_78f4b2';
const createTicketBody: CreateTicketBody = {
  title: 'Unable to access project dashboard',
  description: 'Receiving 403 when accessing /dashboard for project X',
  priority: 'high',
  tags: ['dashboard', 'access'] // שדה אופציונלי להדגמה
};
const result: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]

---