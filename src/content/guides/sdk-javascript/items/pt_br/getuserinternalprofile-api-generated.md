## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserInternalProfile'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const ssoToken: string = "sso_abcde12345";

  const profileOnly: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId);
  const profileWithComment: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId);
  const fullProfile: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId, ssoToken);
}

demo();
[inline-code-end]

---