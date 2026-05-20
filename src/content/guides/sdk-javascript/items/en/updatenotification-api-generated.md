## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateNotificationBody | UpdateNotificationBody | Yes |  |
| userId | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateNotification Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme123";
const id: string = "notif_7w9c21";
const updateNotificationBody: UpdateNotificationBody = {
  status: "resolved",
  reason: "user_reported_spam",
  notes: "Auto-moderation validated spam content"
};
const userId: string | undefined = "moderator_42";
const result: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
[inline-code-end]
