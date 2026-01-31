## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigs200Response.ts)

## Example

[inline-code-attrs-start title = 'getDomainConfigs Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-tenant-12";
const response: GetDomainConfigs200Response = await getDomainConfigs(tenantId);
const firstHostname: string | undefined = (response as GetDomainConfigs200Response & { domains?: { hostname?: string }[] }).domains?.[0]?.hostname;
const domainsCount: number | undefined = (response as GetDomainConfigs200Response & { domains?: unknown[] }).domains?.length;
[inline-code-end]
