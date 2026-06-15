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
const tenantId: string = "tenant_0a1b2c3d";
const id: string = "user_984321";
const redirectURL: string = "https://app.acme-corp.com/welcome";
const responseWithRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
const responseWithoutRedirect: FlagCommentPublic200Response = await sendLoginLink(tenantId, id);
[inline-code-end]
