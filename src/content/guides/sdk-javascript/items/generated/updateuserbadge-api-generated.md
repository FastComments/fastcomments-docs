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
const tenantId: string = "tenant_9k2h7a";
const id: string = "badge_01F8ZC7";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  name: "Helpful Contributor",
  description: "Awarded for 100 helpful comments",
  iconUrl: "https://cdn.fastcomments.com/badges/helpful.png",
  enabled: true,
  expiresAt: "2026-12-31T23:59:59Z" // optional parameter
} as UpdateUserBadgeParams;
const result: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]
