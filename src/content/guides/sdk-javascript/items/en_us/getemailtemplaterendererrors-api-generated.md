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
const tenantId: string = 'acme-tenant-01';
const id: string = 'tmpl_7f9a2b4c';
const skip: number = 20;

const errorsWithSkip: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, id, skip);
const errorsFirstPage: GetEmailTemplateRenderErrors200Response = await getEmailTemplateRenderErrors(tenantId, id);
[inline-code-end]
