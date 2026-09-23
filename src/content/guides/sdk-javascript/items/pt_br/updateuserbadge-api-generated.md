## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Sim |  |

## Resposta

Retorna: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo updateUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f8b7c6d";
const id: string = "user-4a3b2c1d";

const updateParams: UpdateUserBadgeParams = {
  badgeId: "badge-premium",
  // campo opcional
  expiresAt: new Date("2025-12-31T23:59:59Z")
};

const result: APIEmptySuccessResponse = await updateUserBadge(tenantId, id, updateParams);
[inline-code-end]

---