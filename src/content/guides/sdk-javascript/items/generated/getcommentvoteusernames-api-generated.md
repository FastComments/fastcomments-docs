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
(async () => {
  const tenantId: string = 'fc_tenant_42';
  const commentId: string = 'cmt_9f8e7d6';
  const dir: number = 1;
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0MjMiLCJlbWFpbCI6InVzZXJAZXhhbXBsZS5jb20ifQ.signature';

  const result: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
  const resultWithSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir, sso);

  console.log(result, resultWithSSO);
})();
[inline-code-end]
