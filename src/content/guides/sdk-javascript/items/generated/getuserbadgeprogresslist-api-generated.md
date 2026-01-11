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
const tenantId: string = '3fa85f64-5717-4562-b3fc-2c963f66afa6';
const userId: string = 'user-8d7b2a9c';
const limit: number = 25;
const skip: number = 0;

const badgeProgress: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId, userId, limit, skip);
const badgeProgressMinimal: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId);
[inline-code-end]
