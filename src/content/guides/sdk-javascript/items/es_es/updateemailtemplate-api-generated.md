## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "New Comment Notification",
    // htmlContent es opcional y omitido
    status: { code: 200, message: "Active" } // APIStatus
  };

  const response: APIEmptyResponse = await updateEmailTemplate(tenantId, templateId, updateBody);
  console.log(response);
}
[inline-code-end]

---