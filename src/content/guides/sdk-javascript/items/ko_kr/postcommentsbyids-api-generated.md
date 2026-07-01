## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`PostCommentsByIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostCommentsByIdsResponse.ts)

## 예시

[inline-code-attrs-start title = 'postCommentsByIds 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentsByIdsParams: CommentsByIdsParams = {
  commentIds: ['cmt001', 'cmt002'],
  includeUserInfo: true,
  includeThreadInfo: false,
} as CommentsByIdsParams;

const fullResponse: PostCommentsByIdsResponse = await postCommentsByIds(
  commentsByIdsParams,
  'tenant-12345',
  'sso-token-xyz'
);

const minimalResponse: PostCommentsByIdsResponse = await postCommentsByIds(
  commentsByIdsParams
);
[inline-code-end]