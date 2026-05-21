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
async function fetchDeletedEventOptional(tenantId: string, id: string, includeRelated?: boolean): Promise<FlagCommentPublic200Response> {
  const res: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
  if (includeRelated) { /* optionally process related data */ }
  return res;
}

const tenantId: string = 'acme-corp-tenant-42';
const webhookId: string = 'wh_evt_20260520_9f2b7';
const deletedEvent: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, webhookId);
[inline-code-end]
