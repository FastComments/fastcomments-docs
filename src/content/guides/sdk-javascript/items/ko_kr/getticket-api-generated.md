## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| userId | string | 아니요 |  |

## 응답

반환: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicket200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTicket 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const ticketId: string = 'TCKT-20250615-42';
const userId: string = 'user_84b2';

const ticketWithUser: GetTicket200Response = await getTicket(tenantId, ticketId, userId);
const ticketWithoutUser: GetTicket200Response = await getTicket(tenantId, ticketId);

console.log(ticketWithUser.id, ticketWithoutUser.id);
[inline-code-end]

---