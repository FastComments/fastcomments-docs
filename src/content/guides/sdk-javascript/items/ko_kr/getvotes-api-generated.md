## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## 예제

[inline-code-attrs-start title = 'getVotes 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f8e91c2';
const urlId: string = 'https://www.sportsdaily.com/news/2026/06/15/championship-game-recap';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
console.log(votes);
[inline-code-end]

---