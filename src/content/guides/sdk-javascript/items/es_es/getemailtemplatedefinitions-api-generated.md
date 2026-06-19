## Parámetros

| Nombre | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |

## Respuesta

Devuelve: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// Los parámetros opcionales (si se admiten) podrían pasarse como segundo argumento, por ejemplo getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]

---