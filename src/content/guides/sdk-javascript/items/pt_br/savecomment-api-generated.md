## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| createCommentParams | CreateCommentParams | Sim |  |
| isLive | boolean | Não |  |
| doSpamCheck | boolean | Não |  |
| sendEmails | boolean | Não |  |
| populateNotifications | boolean | Não |  |

## Resposta

Retorna: [`SaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'saveComment Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    true,   // isLive
    false   // doSpamCheck
  );
  console.log(response);
}
submitComment();
[inline-code-end]

---