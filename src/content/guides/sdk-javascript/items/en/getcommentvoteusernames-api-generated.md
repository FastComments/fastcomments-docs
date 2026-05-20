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
const tenantId: string = 'tenant_42';
const commentId: string = 'cmt_987654321';
const dir: number = -1;
const sso: string | undefined = 'sso:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

(async () => {
  const result: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir, sso);
  console.log(result);
})();
[inline-code-end]
