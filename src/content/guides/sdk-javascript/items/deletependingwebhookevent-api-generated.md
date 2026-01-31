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
const tenantId: string = "tenant_7f2b3c9d";
const id: string = "wh_evt_20260109_ab12c3";
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
console.log(result);
[inline-code-end]
