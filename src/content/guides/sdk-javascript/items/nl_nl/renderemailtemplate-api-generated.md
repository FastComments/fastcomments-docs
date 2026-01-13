## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Ja |  |
| locale | string | Nee |  |

## Antwoord

Retourneert: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'renderEmailTemplate Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-7f3';
const renderEmailTemplateBody: RenderEmailTemplateBody = {
  templateId: 'new-comment-notification',
  recipientEmail: 'jane.doe@acme.com',
  variables: {
    commenterName: 'Samir Patel',
    commentText: 'I found this article really helpful — thanks for sharing!',
    threadUrl: 'https://acme.com/blog/123#comments'
  },
  includeUnsubscribeLink: true
};
const locale: string = 'en-US';
const response: RenderEmailTemplate200Response = await renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
[inline-code-end]

---