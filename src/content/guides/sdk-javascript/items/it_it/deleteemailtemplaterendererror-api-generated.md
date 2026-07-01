## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| errorId | string | Sì |  |

## Risposta

Restituisce: [`DeleteEmailTemplateRenderErrorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateRenderErrorResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---