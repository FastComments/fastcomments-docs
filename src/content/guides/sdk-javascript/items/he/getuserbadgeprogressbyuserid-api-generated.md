## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | כן |  |

## Response

מחזיר: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserBadgeProgressByUserId'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9c2d3b';
const maybeUserId: string | undefined = 'user_4b8e1f9a'; // מקור אופציונלי (יכול להיות undefined)
const userId: string = maybeUserId ?? 'user_fallback0001';
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);
console.log(result);
[inline-code-end]

---