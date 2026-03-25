## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |

## 回應

回傳: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## 範例

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9c2d3b';
const maybeUserId: string | undefined = 'user_4b8e1f9a'; // 可選來源（可能為 undefined）
const userId: string = maybeUserId ?? 'user_fallback0001';
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);
console.log(result);
[inline-code-end]

---