## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'deleteUserBadge Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const badgeId: string = "moderator-badge";
  const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
  console.log(result);
})();
[inline-code-end]