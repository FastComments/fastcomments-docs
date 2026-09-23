## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentsByIdsParams | CommentsByIdsParams | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postCommentsByIds Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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