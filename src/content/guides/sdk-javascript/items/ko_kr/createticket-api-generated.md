## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## 응답

반환: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## 예시

[inline-code-attrs-start title = 'createTicket 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string은 선택 사항이며 생략되었습니다
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// 응답의 선택적 필드를 사용하는 예시
// console.log(response.ticket?.id);
[inline-code-end]