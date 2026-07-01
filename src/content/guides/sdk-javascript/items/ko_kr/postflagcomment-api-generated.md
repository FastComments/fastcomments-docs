## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`PostFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostFlagCommentResponse.ts)

## 예시

[inline-code-attrs-start title = 'postFlagComment 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_20230915_001";
const broadcastId: string = "brd_20230915_live";
const tenantId: string = "tenant_42";
const sso: string = "sso_token_abc123";

const flaggedResponse: PostFlagCommentResponse = await postFlagComment(
  commentId,
  broadcastId,
  tenantId,
  sso
);
[inline-code-end]