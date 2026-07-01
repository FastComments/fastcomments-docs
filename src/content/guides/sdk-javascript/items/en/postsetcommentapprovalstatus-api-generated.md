## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| approved | boolean | No |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`PostSetCommentApprovalStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentApprovalStatusResponse.ts)

## Example

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main() {
  const commentIdOnly: string = "cmt_1001";
  const resultOnly: PostSetCommentApprovalStatusResponse = await postSetCommentApprovalStatus(commentIdOnly);

  const commentIdFull: string = "cmt_2002";
  const approvedFull: boolean = true;
  const broadcastIdFull: string = "brd_3003";
  const tenantIdFull: string = "tenant_abc";
  const ssoFull: string = "sso_token_xyz";
  const resultFull: PostSetCommentApprovalStatusResponse = await postSetCommentApprovalStatus(
    commentIdFull,
    approvedFull,
    broadcastIdFull,
    tenantIdFull,
    ssoFull
  );

  console.log(resultOnly, resultFull);
}
main();
[inline-code-end]
