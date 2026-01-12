## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigs200Response.ts)

## Example

[inline-code-attrs-start title = 'getDomainConfigs Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// getDomainConfigs only requires tenantId; there are no optional parameters for this call
const tenantId: string = "acme-corp-001";
const domainConfigs: GetDomainConfigs200Response = await getDomainConfigs(tenantId);
[inline-code-end]
