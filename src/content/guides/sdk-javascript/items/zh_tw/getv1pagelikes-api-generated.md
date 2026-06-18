## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回傳

回傳：[`GetV1PageLikes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes200Response.ts)

## 範例

[inline-code-attrs-start title = 'getV1PageLikes 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-742';
const urlId: string = 'sports/2026/06/15/world-cup-preview';
const pageLikes: GetV1PageLikes200Response = await getV1PageLikes(tenantId, urlId);
[inline-code-end]

---