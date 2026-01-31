## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Yes |  |
| locale | string | No |  |

## Response

Returns: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## Example

[inline-code-attrs-start title = 'renderEmailTemplate Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'fc_tenant_67890';
  const renderEmailTemplateBody: RenderEmailTemplateBody = {
    templateId: 'comment-reply',
    recipient: { name: 'Jordan Smith', email: 'jordan.smith@acme.co' },
    comment: { id: 'cmt_abc123', text: 'I agree with your point.' },
    placeholders: { threadTitle: 'Feature discussion' },
    actionUrl: 'https://acme.co/discussion/thread/42'
  } as RenderEmailTemplateBody;
  const resultDefault: RenderEmailTemplate200Response = await renderEmailTemplate(tenantId, renderEmailTemplateBody);
  const resultFr: RenderEmailTemplate200Response = await renderEmailTemplate(tenantId, renderEmailTemplateBody, 'fr-FR');
  console.log(resultDefault, resultFr);
})();
[inline-code-end]
