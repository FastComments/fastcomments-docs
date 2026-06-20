## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| commentId | string | Sim |  |
| banEmail | boolean | Não |  |
| banEmailDomain | boolean | Não |  |
| banIP | boolean | Não |  |
| deleteAllUsersComments | boolean | Não |  |
| bannedUntil | string | Não |  |
| isShadowBan | boolean | Não |  |
| updateId | string | Não |  |
| banReason | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Exemplo

[inline-code-attrs-start title = 'postBanUserFromComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // bloquearEmail
  false,       // bloquearDominioEmail
  true,        // bloquearIP
  true,        // deletarTodosOsComentariosDoUsuario
  bannedUntil,
  false,       // banimentoOculto
  updateId,
  banReason,
  sso
);
[inline-code-end]

---