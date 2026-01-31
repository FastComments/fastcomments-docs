## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Example

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
const firstTemplate: EmailTemplateDefinition | undefined = Array.isArray(response) ? response[0] : undefined;
const firstTemplateName: string | undefined = firstTemplate?.name;
console.log('Templates loaded:', Array.isArray(response) ? response.length : 0, 'First template:', firstTemplateName);
[inline-code-end]
