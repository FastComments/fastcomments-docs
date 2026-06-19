## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| limit | number | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressListResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserBadgeProgressList'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant-01";
  const userId: string | undefined = "user-9e12b4";
  const limit: number | undefined = 20;
  const skip: number | undefined = 0;
  const result: APIGetUserBadgeProgressListResponse = await getUserBadgeProgressList(tenantId, userId, limit, skip);
  console.log(result);
})();
[inline-code-end]

---