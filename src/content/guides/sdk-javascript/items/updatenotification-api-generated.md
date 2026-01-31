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
const tenantId: string = 'tenant_7f3b2a1c';
const id: string = 'notification_9b8c7d6a';
const updateNotificationBody: UpdateNotificationBody = {
  enabled: true,
  channels: ['email', 'push'],
  frequency: 'immediate'
};
const response: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, 'user_42');
[inline-code-end]
