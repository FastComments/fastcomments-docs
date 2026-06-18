---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |

## 回應

回傳: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressList200Response.ts)

## 範例

[inline-code-attrs-start title = 'getUserBadgeProgressList 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3a2b9c';
const userId: string = 'user_7721d';
const limit: number = 20;
const skip: number = 0;
const result: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId, userId, limit, skip);
[inline-code-end]

---