## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| errorId | string | Da |  |

## Odgovor

Vrne: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a1d2f9b";
const id: string = "email_template_42b1";
const errorId: string = "render_err_2026-04-24_7f3c";
const includeStackTrace: boolean | undefined = undefined; // primer neobveznega parametra

const response: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
// Če bi bil podprt neobvezen objekt možnosti, bi lahko izgledal takole:
// await deleteEmailTemplateRenderError(tenantId, id, errorId /*, { includeStackTrace } */);
[inline-code-end]

---