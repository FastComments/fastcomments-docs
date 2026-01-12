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
async function example(): Promise<void> {
  const tenantId: string = 'acme_corp';
  const commentId: string = 'cmt_000123456';
  const dirDownvote: number = -1;
  const voteUserNamesDown: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dirDownvote);
  const dirUpvote: number = 1;
  const ssoToken: string = 'sso:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const voteUserNamesUp: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dirUpvote, ssoToken);
  console.log(voteUserNamesDown, voteUserNamesUp);
}
example();
[inline-code-end]
