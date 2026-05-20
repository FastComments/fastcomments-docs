## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Example

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprise-47';
const response: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
const firstTemplate: EmailTemplateDefinition | undefined = (response as unknown as {templates?: EmailTemplateDefinition[]}).templates?.[0];
[inline-code-end]
