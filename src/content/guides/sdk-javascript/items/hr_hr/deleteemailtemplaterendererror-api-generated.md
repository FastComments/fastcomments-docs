## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| errorId | string | Da |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeRenderError() {
  const tenantId: string = "acme-corp-tenant";
  const templateId: string = "welcome-email-template";
  const errorId: string = "render-err-20230915";

  const result: APIEmptyResponse = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
}
[inline-code-end]