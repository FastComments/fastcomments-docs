## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |
| state | number | 아니오 |  |
| skip | number | 아니오 |  |
| limit | number | 아니오 |  |

## 응답

반환: [`GetTicketsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse1.ts)

## 예시

[inline-code-attrs-start title = 'getTickets 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadTickets() {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";
  const state: number = 2; // 예: 닫힌 상태
  const skip: number = 10;
  const limit: number = 5;

  const ticketsFull: GetTicketsResponse1 = await getTickets(tenantId, userId, state, skip, limit);
  const ticketsPartial: GetTicketsResponse1 = await getTickets(tenantId);
}

loadTickets();
[inline-code-end]