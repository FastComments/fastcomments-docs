## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCachedNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const id: string = 'user_00012345';
const includeUnreadOnly: boolean | undefined = true; // indicador de parâmetro opcional (demonstrado)
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, id);
[inline-code-end]

---