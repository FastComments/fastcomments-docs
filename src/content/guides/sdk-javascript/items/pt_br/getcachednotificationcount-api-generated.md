## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCountResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCachedNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3a2b4c";
const id: string = "user_9812b";
const result: GetCachedNotificationCountResponse = await getCachedNotificationCount(tenantId, id);
[inline-code-end]

---