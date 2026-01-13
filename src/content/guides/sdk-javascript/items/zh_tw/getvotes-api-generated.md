## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

回傳: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## 範例

[inline-code-attrs-start title = 'getVotes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3c_prod';
const urlId: string = '/news/2026/typescript-ecosystem-update';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
// 如果存在可選參數，例如 includeHidden，可能會像這樣使用：
// const votesWithHidden: GetVotes200Response = await getVotes(tenantId, urlId, { includeHidden: true });
[inline-code-end]

---