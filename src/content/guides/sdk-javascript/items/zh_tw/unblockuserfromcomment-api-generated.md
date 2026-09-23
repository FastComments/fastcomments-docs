## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## 範例

[inline-code-attrs-start title = 'unBlockUserFromComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const commentId: string = "comment-4a3b2c1d";
  const params: UnBlockFromCommentParams = {
    reason: "User appealed the block",
    adminNote: "Reviewed and unblocked"
  };
  const userId: string = "user-5e6f7g8h";
  const anonUserId: string = "anon-1a2b3c4d";
  const result: UnblockSuccess = await unBlockUserFromComment(
    tenantId,
    commentId,
    params,
    userId,
    anonUserId
  );
  console.log(result);
})();
[inline-code-end]