## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| contextUserId | string | Não |  |
| isLive | boolean | Não |  |

## Resposta

Retorna: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResult.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeComment(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const commentId: string = "comment_9876";
    const contextUserId: string = "user_42";
    const result: DeleteCommentResult = await deleteComment(tenantId, commentId, contextUserId, true);
    console.log(result);
}
removeComment();
[inline-code-end]

---