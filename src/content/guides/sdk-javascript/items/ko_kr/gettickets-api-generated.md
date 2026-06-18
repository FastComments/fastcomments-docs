## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |
| state | number | 아니오 |  |
| skip | number | 아니오 |  |
| limit | number | 아니오 |  |

## 응답

반환: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTickets200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTickets 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const userId: string = 'user_87b3';
const state: number = 2;
const skip: number = 0;
const limit: number = 50;

const tickets: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
[inline-code-end]

---