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
const tenantId: string = 'acme-tenant-42';
const templateId: string = 'password-reset-v1';
const skip: number = 10;
const pageWithSkip: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, templateId, skip);
const firstPage: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, templateId);
[inline-code-end]
