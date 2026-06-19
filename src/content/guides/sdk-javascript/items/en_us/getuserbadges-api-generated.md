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

Returns: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgesResponse.ts)

## Example

[inline-code-attrs-start title = 'getUserBadges Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a1c9f2b";
const userId: string = "user_4b2d1e9a";
const badgeId: string = "badge_gold_01";
const type: number = 2;
const displayedOnComments: boolean = true;
const limit: number = 25;
const skip: number = 0;

const response: APIGetUserBadgesResponse = await getUserBadges(
  tenantId,
  userId,
  badgeId,
  type,
  displayedOnComments,
  limit,
  skip
);
[inline-code-end]
