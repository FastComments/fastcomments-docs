## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Sim |  |
| locale | string | Não |  |

## Resposta

Retorna: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de renderEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-987';
const renderEmailTemplateBody: RenderEmailTemplateBody = {
  templateId: 'user-invite',
  subject: "You're invited to Acme",
  placeholders: { firstName: 'Alex' },
  metadata: { source: 'signup-flow' }
};
const locale: string = 'en-US';
const result: RenderEmailTemplate200Response = await renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
[inline-code-end]