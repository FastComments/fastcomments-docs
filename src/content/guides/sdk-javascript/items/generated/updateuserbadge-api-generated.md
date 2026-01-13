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
const tenantId: string = "acme_corp_tenant_01";
const id: string = "badge_000198";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  name: "Top Reviewer",
  description: "Awarded for 100+ helpful reviews",
  color: "#ffb347",
  imageUrl: "https://assets.acme.com/badges/top-reviewer.png",
  // optional: expiresAt can be omitted if badge is permanent
  expiresAt: "2027-01-01T00:00:00Z"
};
const updatedBadge: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]
