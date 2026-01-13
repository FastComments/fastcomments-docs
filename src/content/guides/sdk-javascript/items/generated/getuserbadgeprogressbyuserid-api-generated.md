## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-88a1';
const userId: string = 'user_5f4d2c';
const overrideUserId?: string = undefined; // optional override, demonstrate optional parameter usage
const targetUserId: string = overrideUserId ?? userId;
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, targetUserId);
[inline-code-end]
