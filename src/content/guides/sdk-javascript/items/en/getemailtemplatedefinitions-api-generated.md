## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Example

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-78";
const response: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId, undefined);
[inline-code-end]
