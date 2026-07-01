## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| errorId | string | Yes |  |

## Απόκριση

Επιστρέφει: [`DeleteEmailTemplateRenderErrorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateRenderErrorResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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