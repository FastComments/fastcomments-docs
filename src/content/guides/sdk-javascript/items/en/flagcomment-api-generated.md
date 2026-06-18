## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Example

[inline-code-attrs-start title = 'flagComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f21c9a";
const commentId: string = "cmt_7a12b3e9";
const userId: string = "user_82bd123";
const result: FlagComment200Response = await flagComment(tenantId, commentId, userId);
[inline-code-end]
