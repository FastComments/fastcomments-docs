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
const tenantId: string = 'b3f1a2c4-7e9d-4b2a-8c1f-1234567890ab';
const userId: string = 'user_98765';
const includeArchived: boolean | undefined = undefined; // optional flag example

const badgeProgress: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);

console.log(badgeProgress);
[inline-code-end]
