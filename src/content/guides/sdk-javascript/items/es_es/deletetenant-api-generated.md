## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sure | string | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2b9c8a';
const id: string = 'site_9d4f2a1b';
const confirmation: string = 'CONFIRM_DELETE';
const result: APIEmptyResponse = await deleteTenant(tenantId, id, confirmation);
[inline-code-end]

---