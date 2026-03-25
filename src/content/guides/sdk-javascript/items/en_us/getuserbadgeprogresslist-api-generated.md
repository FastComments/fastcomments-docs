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
(async () => {
  const tenantId: string = 'tenant_4f8c2b9d';
  const userId: string = 'user_9a7e215c';
  const limit: number = 25;
  const skip: number = 0;
  const resultMinimal: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId);
  const resultFull: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId, userId, limit, skip);
  console.log(resultMinimal, resultFull);
})();
[inline-code-end]
