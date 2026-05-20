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
const tenantId: string = 'acme-tenant-782';
const templates: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
const templatesPaged: GetEmailTemplates200Response = await getEmailTemplates(tenantId, 20);
[inline-code-end]
