## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| commentId | string | Sim |  |
| tenantId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetBanUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBanUsersFromCommentResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getBanUsersFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetBanUsers() {
  const commentId: string = "cmt_5f8e3a9b2d";
  const tenantId: string = "tenant_42";
  const sso: string = "sso_token_abc123";

  // Chamar com todos os parâmetros
  const fullResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId, tenantId, sso);
  console.log(fullResult);

  // Chamar apenas com o parâmetro obrigatório
  const minimalResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  console.log(minimalResult);
}

demoGetBanUsers();
[inline-code-end]

---