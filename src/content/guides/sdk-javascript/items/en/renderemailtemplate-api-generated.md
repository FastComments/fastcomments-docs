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
const tenantId: string = 'acme-tenant-01';
const renderEmailTemplateBody: RenderEmailTemplateBody = {
  templateId: 'account_welcome_v2',
  templateData: { firstName: 'Jordan', accountUrl: 'https://app.acme.com/settings' },
  options: { includeUnsubscribe: true }
};
const result: RenderEmailTemplate200Response = await renderEmailTemplate(tenantId, renderEmailTemplateBody, 'en-US');
[inline-code-end]
