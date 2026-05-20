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
const tenantId: string = "acme-enterprises";
const userId: string = "u-55213";

async function fetchUserBadge(tenant: string, id: string, includeHistory?: boolean): Promise<GetUserBadgeProgressById200Response> {
  const progress: GetUserBadgeProgressById200Response = await getUserBadgeProgressById(tenant, id);
  if (includeHistory) { /* optionally process historical entries */ }
  return progress;
}

const badgeProgress: GetUserBadgeProgressById200Response = await getUserBadgeProgressById(tenantId, userId);
[inline-code-end]
