## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## 예시

[inline-code-attrs-start title = 'unBlockUserFromComment 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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