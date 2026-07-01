## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## 예시

[inline-code-attrs-start title = 'postUnFlagComment 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // 댓글 ID
    "brd_67890",          // broadcastId (선택사항)
    "tenant_abc",         // tenantId (선택사항)
    "sso_user_token_789"  // sso (선택사항)
  );
};
[inline-code-end]