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
(async () => {
  const tenantId: string = 'acme-tenant-01';
  const userId: string | undefined = 'user_9876';
  const badgeId: string | undefined = undefined;
  const typeParam: number | undefined = 2;
  const displayedOnComments: boolean | undefined = true;
  const limit: number | undefined = 50;
  const skip: number | undefined = 0;
  const result: GetUserBadges200Response = await getUserBadges(tenantId, userId, badgeId, typeParam, displayedOnComments, limit, skip);
  console.log(result);
})();
[inline-code-end]
