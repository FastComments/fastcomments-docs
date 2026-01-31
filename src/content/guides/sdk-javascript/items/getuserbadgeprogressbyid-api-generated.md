## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressById Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-42";
const badgeId: string = "user-8a3f2c";
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressById(tenantId, badgeId);
const includeDetails: boolean | undefined = true;
if (includeDetails) {
  const progressSummary: UserBadgeProgress | undefined = undefined;
  console.log("Details requested for", badgeId);
}
console.log(result);
[inline-code-end]
