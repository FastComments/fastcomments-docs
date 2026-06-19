## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| errorId | string | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-7f3a2b';
const templateId: string = 'tmpl-9c3e1a2b';
const errorId: string = 'err-2026-06-19-001';
const result: APIEmptyResponse = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]