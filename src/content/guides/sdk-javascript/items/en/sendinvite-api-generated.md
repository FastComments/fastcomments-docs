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
const tenantId: string = "org-5821";
const id: string = "flag-9c4b";
const fromName: string = "Aisha Patel";
const optionalNote: string | undefined = "Please review this flagged comment when you have a moment";
const result: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName, optionalNote);
[inline-code-end]
