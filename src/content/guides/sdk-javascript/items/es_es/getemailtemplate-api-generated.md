## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTemplate() {
  const tenantId: string = "acme-corp-123";
  const templateId: string = "welcome-email-456";
  const response: GetEmailTemplateResponse = await getEmailTemplate(tenantId, templateId);
  const status: APIStatus | undefined = response.status;
  const customTemplate: CustomEmailTemplate | undefined = response.template;
}
[inline-code-end]