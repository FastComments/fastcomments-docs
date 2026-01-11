## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'getDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const region: string | undefined = 'eu';
const tenantId: string = region ? `acme-tenant-82a3f-${region}` : 'acme-tenant-82a3f';
const domain: string = 'comments.acme-corp.com';
const previewMode: boolean | undefined = true;
const response: GetDomainConfig200Response = await getDomainConfig(tenantId, domain);
[inline-code-end]
