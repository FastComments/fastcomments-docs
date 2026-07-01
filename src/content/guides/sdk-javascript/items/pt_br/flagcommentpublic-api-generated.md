## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| isFlagged | boolean | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`FlagCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublicResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo flagCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoFlagComment() {
    const tenantId: string = "acme-tenant-123";
    const commentId: string = "comment-987654";
    const isFlagged: boolean = true;
    const sso: string = "sso-token-abc123";

    const result: FlagCommentPublicResponse = await flagCommentPublic(
        tenantId,
        commentId,
        isFlagged,
        sso
    );

    console.log(result);
}

demoFlagComment();
[inline-code-end]