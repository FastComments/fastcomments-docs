## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'sendInvite Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-128';
const id: string = 'comment-8421f';
const fromName: string = 'Marcus Lindström';
const note: string | undefined = undefined; // optional parameter example
const response: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
[inline-code-end]
