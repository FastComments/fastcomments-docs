## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Respuesta

Devuelve: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCachedNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const id: string = 'user_00012345';
const includeUnreadOnly: boolean | undefined = true; // bandera de parámetro opcional (demostrado)
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, id);
[inline-code-end]

---