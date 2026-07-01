## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | CreateCommentParams | Oui |  |
| isLive | boolean | Non |  |
| doSpamCheck | boolean | Non |  |
| sendEmails | boolean | Non |  |
| populateNotifications | boolean | Non |  |

## Réponse

Renvoie : [`SaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de saveComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitComment() {
  const tenantId: string = "tenant_9f8e7d6c";
  const commentParams: CreateCommentParams = {
    text: "Great post, thanks for sharing!",
    authorId: "user_123abc",
    mentions: [] as CommentUserMentionInfo[],
    hashtags: [] as CommentUserHashTagInfo[]
  };
  const response: SaveCommentResponse = await saveComment(
    tenantId,
    commentParams,
    true,   // en direct
    false   // vérifier le spam
  );
  console.log(response);
}
submitComment();
[inline-code-end]

---