## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| deleteComments | string | Não |  |
| commentDeleteMode | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'deleteTenantUser Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "u-112233";

  const result1: APIEmptyResponse = await deleteTenantUser(tenantId, userId);
  const result2: APIEmptyResponse = await deleteTenantUser(tenantId, userId, "true", "hard");

  console.log(result1, result2);
})();
[inline-code-end]

---