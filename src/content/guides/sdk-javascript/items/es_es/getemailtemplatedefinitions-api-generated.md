## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |

## Respuesta

Devuelve: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_eu_01';
const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
console.log('Email template definitions loaded for', tenantId, templates);
[inline-code-end]

---