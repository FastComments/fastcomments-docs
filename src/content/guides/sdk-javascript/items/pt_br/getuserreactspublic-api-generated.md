## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| postIds | Array<string> | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetUserReactsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublicResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserReactsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const postIds: string[] = ["post_1a2b3c", "post_4d5e6f"];
  const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

  const fullResponse: GetUserReactsPublicResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
  const minimalResponse: GetUserReactsPublicResponse = await getUserReactsPublic(tenantId);
}

demo();
[inline-code-end]