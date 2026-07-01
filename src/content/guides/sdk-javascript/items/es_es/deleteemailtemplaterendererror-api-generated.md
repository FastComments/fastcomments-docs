## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| errorId | string | Sí |  |

## Respuesta

Devuelve: [`DeleteEmailTemplateRenderErrorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateRenderErrorResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function executeDelete() {
  const tenantId: string = "tenant_12345";
  const templateId: string = "email_tpl_001";
  const errorId: string = "render_err_2023";

  const result: DeleteEmailTemplateRenderErrorResponse = await deleteEmailTemplateRenderError(
    tenantId,
    templateId,
    errorId
  );

  console.log(result);
}

executeDelete();
[inline-code-end]