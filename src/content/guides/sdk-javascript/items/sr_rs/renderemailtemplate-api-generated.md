## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Да |  |
| locale | string | Не |  |

## Одговор

Враћа: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример renderEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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