Enable or disable notifications for a specific comment.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | Yes |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Example

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f1b2c";
const notificationId: string = "notification_1024";
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum = UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OptedIn;
const commentId: string = "cmt_20251122_abcd";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature";

const result: UpdateUserNotificationStatus200Response = await updateUserNotificationCommentSubscriptionStatus(
  tenantId,
  notificationId,
  optedInOrOut,
  commentId,
  sso
);
[inline-code-end]
