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
  const tenantId: string = 'site_8a12c9f';
  const displayedOnComments: boolean = true;
  const limit: number = 15;
  const skip: number = 0;
  const response: GetUserBadges200Response = await getUserBadges(tenantId, undefined, undefined, undefined, displayedOnComments, limit, skip);
  console.log(response);
})();
[inline-code-end]
