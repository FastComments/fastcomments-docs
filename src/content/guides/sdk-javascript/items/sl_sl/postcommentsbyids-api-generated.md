## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | Da |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`PostCommentsByIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostCommentsByIdsResponse.ts)

## Primer

[inline-code-attrs-start title = 'postCommentsByIds Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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