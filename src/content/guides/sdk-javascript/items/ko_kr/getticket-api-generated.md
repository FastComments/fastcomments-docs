## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## 응답

Returns: [`GetTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse1.ts)

## 예시

[inline-code-attrs-start title = 'getTicket 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-inc";
const ticketId: string = "ticket-3421";
const userId: string = "alice.smith";

const ticketWithUser: GetTicketResponse1 = await getTicket(tenantId, ticketId, userId);
const ticketWithoutUser: GetTicketResponse1 = await getTicket(tenantId, ticketId);
[inline-code-end]