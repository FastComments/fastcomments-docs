## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vraća: [`DeleteEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateResponse.ts)

## Primjer

[inline-code-attrs-start title = 'deleteEmailTemplate Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_abcde";

  const response: DeleteEmailTemplateResponse = await deleteEmailTemplate(tenantId, templateId);

  // Primjer pristupa opcionalnom svojstvu iz odgovora
  const statusCode: number | undefined = response.status?.code;
}();
[inline-code-end]