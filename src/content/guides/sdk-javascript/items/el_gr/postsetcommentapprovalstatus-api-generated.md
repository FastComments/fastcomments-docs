## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| commentId | string | Ναι |  |
| approved | boolean | Όχι |  |
| broadcastId | string | Όχι |  |
| tenantId | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Returns: [`PostSetCommentApprovalStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentApprovalStatusResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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