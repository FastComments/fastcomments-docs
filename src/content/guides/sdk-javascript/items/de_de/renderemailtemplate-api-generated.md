## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Ja |  |
| locale | string | Nein |  |

## Antwort

Gibt zurück: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'renderEmailTemplate Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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