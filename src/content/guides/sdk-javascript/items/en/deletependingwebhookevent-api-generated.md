## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deletePendingWebhookEvent Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'arcadia-enterprises';
const pendingEventId: string | undefined = '3f9b2c8e-6a1d-4c2b-b8e9-5d7a9f0c1e2b';
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, pendingEventId!);
[inline-code-end]
