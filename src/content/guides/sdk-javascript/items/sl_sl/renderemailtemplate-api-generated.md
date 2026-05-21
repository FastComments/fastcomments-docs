## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Da |  |
| locale | string | Ne |  |

## Odgovor

Vrne: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer renderEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---