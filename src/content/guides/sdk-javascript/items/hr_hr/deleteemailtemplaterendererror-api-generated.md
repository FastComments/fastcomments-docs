## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| errorId | string | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a1d2f9b";
const id: string = "email_template_42b1";
const errorId: string = "render_err_2026-04-24_7f3c";
const includeStackTrace: boolean | undefined = undefined; // primjer opcionalne zastavice

const response: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
// Ako bi bio podržan neobavezni objekt opcija, mogao bi izgledati ovako:
// await deleteEmailTemplateRenderError(tenantId, id, errorId /*, { includeStackTrace } */);
[inline-code-end]