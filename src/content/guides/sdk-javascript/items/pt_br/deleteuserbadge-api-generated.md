## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8b3f2c7a";
const badgeIdOptional: string | undefined = Math.random() > 0.5 ? "badge_4f9a21" : undefined;
const id: string = badgeIdOptional ?? "badge_backup_01";
const result: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
[inline-code-end]

---