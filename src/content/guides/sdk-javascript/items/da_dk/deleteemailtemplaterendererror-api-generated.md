## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| errorId | string | Ja |  |

## Svar

Returnerer: [`DeleteEmailTemplateRenderErrorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateRenderErrorResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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