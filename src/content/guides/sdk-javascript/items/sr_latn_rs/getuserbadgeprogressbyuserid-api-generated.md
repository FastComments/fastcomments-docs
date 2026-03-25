## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |

## Odgovor

Vraća: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Primer

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9c2d3b';
const maybeUserId: string | undefined = 'user_4b8e1f9a'; // opcioni izvor (može biti undefined)
const userId: string = maybeUserId ?? 'user_fallback0001';
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);
console.log(result);
[inline-code-end]

---