## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## Example

[inline-code-attrs-start title = 'getEmailTemplates Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7cfa';
const skip: number = 50;
const templatesFirstPage: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
const templatesSecondPage: GetEmailTemplates200Response = await getEmailTemplates(tenantId, skip);
[inline-code-end]
