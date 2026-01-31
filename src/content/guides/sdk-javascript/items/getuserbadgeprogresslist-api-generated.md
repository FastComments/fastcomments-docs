## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| limit | number | No |  |
| skip | number | No |  |

## Response

Returns: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressList200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressList Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8e7d';
const userId: string = 'user_4a12b6';
const limit: number = 20;
const skip: number = 0;
const badgeProgress: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId, userId, limit, skip);
[inline-code-end]
