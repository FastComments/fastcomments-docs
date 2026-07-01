## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| createCommentParams | CreateCommentParams | Sí |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| sendEmails | boolean | No |  |
| populateNotifications | boolean | No |  |

## Respuesta

Devuelve: [`SaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de saveComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    true,   // en vivo
    false   // comprobar spam
  );
  console.log(response);
}
submitComment();
[inline-code-end]