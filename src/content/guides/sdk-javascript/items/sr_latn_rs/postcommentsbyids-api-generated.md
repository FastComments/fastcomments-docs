## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentsByIdsParams | CommentsByIdsParams | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## Primer

[inline-code-attrs-start title = 'postCommentsByIds Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f8b7c6d-1234-5678-90ab-cdef12345678';
const commentsByIdsParams: CommentsByIdsParams = {
  commentIds: ['comment-1', 'comment-2'],
  includeUserBadges: true
};
const responseWithoutSso: ModerationAPIChildCommentsResponse = await postCommentsByIds(tenantId, commentsByIdsParams);
const sso: string = 'sso-abc123def456';
const responseWithSso: ModerationAPIChildCommentsResponse = await postCommentsByIds(tenantId, commentsByIdsParams, sso);
[inline-code-end]