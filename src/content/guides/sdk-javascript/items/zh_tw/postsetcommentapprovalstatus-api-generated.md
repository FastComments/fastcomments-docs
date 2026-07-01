## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| commentId | string | 是 |  |
| approved | boolean | 否 |  |
| broadcastId | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`PostSetCommentApprovalStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostSetCommentApprovalStatusResponse.ts)

## 範例

[inline-code-attrs-start title = 'postSetCommentApprovalStatus 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---