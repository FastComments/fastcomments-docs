## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## Example

[inline-code-attrs-start title = 'getEmailTemplate Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9b3f2a';
const templateId: string = 'tpl_4f8c7a2';
let preferredLocale: string | undefined = 'en-US';
const emailTemplate: GetEmailTemplate200Response = await getEmailTemplate(tenantId, templateId);
const localeToUse: string = preferredLocale ?? 'en-US';
[inline-code-end]
