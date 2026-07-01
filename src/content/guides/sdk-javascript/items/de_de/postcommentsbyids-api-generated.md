## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| commentsByIdsParams | CommentsByIdsParams | Ja |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`PostCommentsByIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostCommentsByIdsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postCommentsByIds Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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