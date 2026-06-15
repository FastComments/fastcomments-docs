---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

回傳: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## 範例

[inline-code-attrs-start title = 'getVotes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f8e91c2';
const urlId: string = 'https://www.sportsdaily.com/news/2026/06/15/championship-game-recap';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
console.log(votes);
[inline-code-end]

---