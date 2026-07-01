## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Відповідь

Повертає: [`PostCommentsByIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostCommentsByIdsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад postCommentsByIds'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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