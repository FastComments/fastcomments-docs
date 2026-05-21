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
const tenantId: string = 'acme-inc';
const id: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
const updateNotificationBody: UpdateNotificationBody = { resolved: true, reason: 'Offensive language detected', notes: 'Marked by moderator' };
const userId: string = 'user_1024';
const resultWithUser: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
const resultWithoutUser: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody);
[inline-code-end]
