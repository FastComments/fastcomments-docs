## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| dir | number | Yes |  |
| sso | string | No |  |

## Response

Returns: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNames200Response.ts)

## Example

[inline-code-attrs-start title = 'getCommentVoteUserNames Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-342";
const commentId: string = "cmt_8f7b3d9a";
const dir: number = 1;
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature";

const resultWithSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir, sso);
const resultWithoutSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
[inline-code-end]
