## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| contextUserId | string | 아니오 |  |
| isLive | boolean | 아니오 |  |

## 응답

반환: [`DeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteComment 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_01';
  const commentId: string = 'comment_abc123';
  const contextUserId: string = 'user_42';
  const isLive: boolean = false;

  const deleteResult: DeleteCommentResponse = await deleteComment(tenantId, commentId, contextUserId, isLive);
  const simpleResult: DeleteCommentResponse = await deleteComment(tenantId, commentId);
})();
[inline-code-end]

---