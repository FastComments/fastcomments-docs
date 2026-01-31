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
const tenantId: string = 'org-7f4c2b9a-5512';
const templateId: string = 'welcome-email-template-v2';
const template: GetEmailTemplate200Response = await getEmailTemplate(tenantId, templateId);
[inline-code-end]
