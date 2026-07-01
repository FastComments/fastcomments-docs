## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Oui |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Oui |  |
| editKey | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`SetCommentTextResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple setCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const commentId: string = 'cmt_98765';
  const broadcastId: string = 'brd_112233';
  const commentTextUpdateRequest: CommentTextUpdateRequest = {
    text: 'Updated comment with @john.doe and #important',
    mentions: [{ userId: 'user_001', username: 'john.doe' }],
    hashtags: [{ tag: 'important' }],
  };
  const editKey: string = 'edit_abc123';
  const result: SetCommentTextResponse1 = await setCommentText(
    tenantId,
    commentId,
    broadcastId,
    commentTextUpdateRequest,
    editKey,
  );
  console.log(result);
})();
[inline-code-end]