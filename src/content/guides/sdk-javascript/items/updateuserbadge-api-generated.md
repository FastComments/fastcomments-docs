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
const tenantId: string = "tenant_8f3a2c";
const id: string = "badge_01A7B9";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  title: "Community Moderator",
  description: "Recognizes consistent, helpful moderation and rule enforcement",
  iconUrl: "https://cdn.fastcomments.com/badges/moderator.svg",
  enabled: true,
  // optional parameter demonstrated
  expiresAt: "2027-12-31T23:59:59Z"
};
const result: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]
