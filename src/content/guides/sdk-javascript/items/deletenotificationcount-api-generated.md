## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteNotificationCount Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-tenant-42';
const maybeNotificationId: string | undefined = undefined;
const notificationIdToDelete: string = maybeNotificationId ?? '3fa85f64-5717-4562-b3fc-2c963f66afa6';
const result: FlagCommentPublic200Response = await deleteNotificationCount(tenantId, notificationIdToDelete);
console.log('Deleted notification count response:', result);
[inline-code-end]
