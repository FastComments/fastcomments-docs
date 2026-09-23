## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| id | string | Yes |  |
| changeTicketStateBody | ChangeTicketStateBody | Yes |  |

## 응답

반환: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## 예시

[inline-code-attrs-start title = 'changeTicketState 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";
  const ticketId: string = "ticket-20230915-001";

  const changeTicketStateBody: ChangeTicketStateBody = {
    // 선택적 필드 예시
    note: "Resolved after investigation"
  };

  const response: ChangeTicketStateResponse = await changeTicketState(
    tenantId,
    userId,
    ticketId,
    changeTicketStateBody
  );

  console.log(response);
})();
[inline-code-end]