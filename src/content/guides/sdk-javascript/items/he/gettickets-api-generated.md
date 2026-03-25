## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | לא |  |
| state | number | לא |  |
| skip | number | לא |  |
| limit | number | לא |  |

## תגובה

מחזיר: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTickets200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTickets'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_92f3b4c1";
const userId: string = "user_742a9f3e";
const state: number = 1;
const skip: number = 0;
const limit: number = 25;
const ticketsFull: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
const ticketsMinimal: GetTickets200Response = await getTickets("tenant_92f3b4c1");
[inline-code-end]

---