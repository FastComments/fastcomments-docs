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
const tenantId: string = 'tenant-8a3b2f';
const subdomain: string | undefined = 'payments'; // optional segment
const domain: string = subdomain ? `${subdomain}.acme-corp.com` : 'acme-corp.com';
const domainConfig: GetDomainConfig200Response = await getDomainConfig(tenantId, domain);
[inline-code-end]
