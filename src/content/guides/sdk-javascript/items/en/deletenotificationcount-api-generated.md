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
const tenantId: string = 'tenant_4f8b2a';
const idSuffix: string | undefined = undefined; // optional fragment (may be omitted)
const id: string = `notif_9c3d7e-${idSuffix ?? 'web'}`;
const result: FlagCommentPublic200Response = await deleteNotificationCount(tenantId, id);
[inline-code-end]
