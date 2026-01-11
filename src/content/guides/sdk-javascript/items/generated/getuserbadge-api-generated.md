## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acmecorp-prod-01';
const userId: string = 'usr_78a9b2c4';
const result: GetUserBadge200Response = await getUserBadge(tenantId, userId);
const options: { includeDetails?: boolean } = { includeDetails: true };
const badgeName: string | undefined = (result as any)?.userBadge?.name;
if (options.includeDetails) {
  const badgeDescription: string | undefined = (result as any)?.userBadge?.description;
  void badgeDescription;
}
[inline-code-end]
