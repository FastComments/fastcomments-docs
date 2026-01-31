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
const tenantId: string = "tenant_newsroom_89f3c";
const notificationId: string = "notif_2026-01-09-a1b2c3";
const commentId: string = "cmt_7f5d2a";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature";
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum =
  UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OptedIn;

const result: UpdateUserNotificationStatus200Response =
  await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]
