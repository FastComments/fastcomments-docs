---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| dir | number | 是 |  |
| sso | string | 否 |  |

## 回應

返回：[`GetCommentVoteUserNamesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCommentVoteUserNames 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetCommentVoteUserNames() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_5f2a1e3b";
  const dir: number = 1; // 升序

  const votesWithoutSSO: GetCommentVoteUserNamesResponse = await getCommentVoteUserNames(
    tenantId,
    commentId,
    dir
  );

  const ssoToken: string = "sso_abcdef123456";
  const votesWithSSO: GetCommentVoteUserNamesResponse = await getCommentVoteUserNames(
    tenantId,
    commentId,
    dir,
    ssoToken
  );

  console.log(votesWithoutSSO, votesWithSSO);
}
[inline-code-end]

---