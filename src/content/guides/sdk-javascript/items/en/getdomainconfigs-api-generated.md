## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigs200Response.ts)

## Example

[inline-code-attrs-start title = 'getDomainConfigs Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const providedTenantId: string | undefined = 'tenant_4f9c2b-8a1';
const tenantId: string = providedTenantId ?? 'tenant_00001-xyz';
const result: GetDomainConfigs200Response = await getDomainConfigs(tenantId);
console.log(result);
[inline-code-end]
