## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |
| createTicketBody | CreateTicketBody | 예 |  |

## 응답

반환: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## 예제

[inline-code-attrs-start title = 'createTicket 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const userId: string = "user_72b9f4";
const createTicketBody: CreateTicketBody = {
  subject: "Subscription renewal failed for card on file",
  description: "Customer's card was declined by the payment processor during automatic renewal. Transaction ID: txn_9a8b7c. Please review gateway logs and retry.",
  priority: "high", // 선택적 필드 예시
  contactEmail: "billing@acme-corp.com", // 선택적 연락처 정보
  relatedUrl: "https://acme-corp.com/account/billing"
};
const ticketResponse: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]