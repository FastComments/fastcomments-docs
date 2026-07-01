## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| badgeId | string | Não |  |
| type | number | Não |  |
| displayedOnComments | boolean | Não |  |
| limit | number | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgesResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = "tenant-01";
  const userId: string = "user-42";
  const badgeId: string = "badge-gold";
  const type: number = 1;
  const displayedOnComments: boolean = true;
  const limit: number = 10;
  const skip: number = 5;

  const fullResult: GetUserBadgesResponse = await getUserBadges(
    tenantId,
    userId,
    badgeId,
    type,
    displayedOnComments,
    limit,
    skip
  );

  const minimalResult: GetUserBadgesResponse = await getUserBadges(tenantId);
}
example();
[inline-code-end]

---