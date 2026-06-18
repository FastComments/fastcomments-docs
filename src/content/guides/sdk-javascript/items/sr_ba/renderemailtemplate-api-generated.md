## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Da |  |
| locale | string | Ne |  |

## Odgovor

Vraća: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer renderEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '7f7e2b90-3a2b-4d9b-9df1-5f0b6b2e8a1c';
const renderEmailTemplateBody: RenderEmailTemplateBody = {
  templateId: 'welcome_email',
  recipient: { email: 'jordan.smith@acme.co', name: 'Jordan Smith' },
  variables: { siteName: 'Acme Forum', verificationUrl: 'https://acme.forum/verify?code=abc123' }
};
const locale: string = 'en-US';
const result: RenderEmailTemplate200Response = await renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
[inline-code-end]

---