## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrors200Response.ts)

## Example

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b3f2c";
const templateId: string = "email_tpl_42";
const skipCount: number = 20;
const resultNoSkip: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, templateId);
const resultWithSkip: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, templateId, skipCount);
[inline-code-end]
