## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | Evet |  |
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döner: [`PostCommentsByIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostCommentsByIdsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'postCommentsByIds Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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