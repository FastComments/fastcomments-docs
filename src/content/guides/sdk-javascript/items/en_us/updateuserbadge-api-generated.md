## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## Response

Returns: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Example

[inline-code-attrs-start title = 'updateUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-web-tenant-7";
const id: string = "badge_48f2a9";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  label: "Community Champion",
  description: "Awarded for exceptional moderation and sustained helpful responses",
  active: true,
  expiresAt: "2026-12-31T23:59:59Z", // optional expiry demonstrated
  notifyUsers: true,
  metadata: { awardedBy: "moderator_jane" }
};
const result: APIEmptySuccessResponse = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]
