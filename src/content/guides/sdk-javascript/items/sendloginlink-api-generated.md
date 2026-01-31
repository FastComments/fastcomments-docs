## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| redirectURL | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'sendLoginLink Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const id: string = 'user_842a1f';
const redirectURL: string = 'https://app.acme.com/welcome';

const loginResponse: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
const loginResponseNoRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id);
[inline-code-end]
