## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| badgeId | string | No |  |
| type | number | No |  |
| displayedOnComments | boolean | No |  |
| limit | number | No |  |
| skip | number | No |  |

## Response

Returns: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadges200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadges Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9b3a2c';
const userId: string = 'user_5821';
const badgeId: string | undefined = undefined;
const type: number | undefined = 2;
const displayedOnComments: boolean | undefined = true;
const limit: number | undefined = 25;
const skip: number | undefined = 0;
const badges: GetUserBadges200Response = await getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip);
[inline-code-end]
