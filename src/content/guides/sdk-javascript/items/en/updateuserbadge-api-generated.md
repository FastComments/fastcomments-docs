## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## Response

Returns: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'updateUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-01";
const id: string = "user_81234";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  badgeId: "top_contributor",
  title: "Top Contributor",
  description: "Awarded for sustained high-quality community contributions",
  expiresAt: "2026-12-31T23:59:59Z" // optional parameter demonstrated
} as UpdateUserBadgeParams;
const result: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]
