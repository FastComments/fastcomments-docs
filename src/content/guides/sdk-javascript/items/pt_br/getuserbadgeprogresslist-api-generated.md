## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| limit | number | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressListResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserBadgeProgressList'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const userId: string = "user-123e4567-e89b-12d3-a456-426614174000";
  const limit: number = 10;
  const skip: number = 0;

  const response: APIGetUserBadgeProgressListResponse = await getUserBadgeProgressList(tenantId, userId, limit, skip);
  const status: APIStatus = response.status;
  const badges: UserBadgeProgress[] = response.badges;
})();
[inline-code-end]