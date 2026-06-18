## Parámetros

| Nombre | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrors200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getEmailTemplateRenderErrors'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const id: string = 'tmpl_7f9a2b4c';
const skip: number = 20;

const errorsWithSkip: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, id, skip);
const errorsFirstPage: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, id);
[inline-code-end]

---