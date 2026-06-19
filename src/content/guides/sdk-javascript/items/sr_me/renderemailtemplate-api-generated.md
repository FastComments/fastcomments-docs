---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Da |  |
| locale | string | Ne |  |

## Odgovor

Vraća: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplateResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer renderEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2c44';
const renderEmailTemplateBody: RenderEmailTemplateBody = {
  templateId: 'welcome_v2',
  recipient: { name: 'Lucas Moreno', email: 'lucas@startup.io' },
  variables: { siteName: 'TechDaily', activationLink: 'https://techdaily.io/activate/abc123' }
};
const locale: string = 'en-US';
const result: RenderEmailTemplateResponse = await renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
[inline-code-end]

---