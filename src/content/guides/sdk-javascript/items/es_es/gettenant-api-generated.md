## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp";
const id: string = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
interface GetOptions { includeDeleted?: boolean; locale?: string; }
const options: GetOptions = { locale: "en-US" };
const result: GetTenant200Response = await getTenant(tenantId, id);
[inline-code-end]

---