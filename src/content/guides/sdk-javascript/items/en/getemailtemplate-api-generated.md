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
const tenantId: string = 'tenant-4a1c2b9f';
const id: string = 'tmpl-7c3b21';
const preferredLocale: string | undefined = 'en-US';

const template: GetEmailTemplate200Response = await getEmailTemplate(tenantId, id);
[inline-code-end]
