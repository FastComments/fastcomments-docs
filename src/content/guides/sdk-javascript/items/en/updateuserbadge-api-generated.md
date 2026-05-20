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
const tenantId: string = "acme-tenant-42";
const id: string = "user-9b3f7c";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  badgeId: "top-contributor",
  reason: "Recognized for sustained exceptional contributions",
  notifyRecipients: true
};
const result: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]
