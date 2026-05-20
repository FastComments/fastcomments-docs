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
const tenantId: string = '6f1b2e9a-3c4d-11ec-8d3d-0242ac130003';
const domain: string = 'billing.acme-corp.com';
const result: GetDomainConfig200Response = await getDomainConfig(tenantId, domain);
[inline-code-end]
