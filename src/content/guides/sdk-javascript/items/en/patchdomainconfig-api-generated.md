## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'patchDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PatchDomainConfigParams { tlsEnabled?: boolean; primaryMx?: string; tags?: Record<string,string> }
interface GetDomainConfig200Response { domain: string; tenantId: string; tlsEnabled: boolean; primaryMx?: string; updatedAt: string; tags?: Record<string,string> }

const tenantId: string = 'acme-tenant-42'
const domainToUpdate: string = 'billing.acme-corp.com'
const patchDomainConfigParams: PatchDomainConfigParams = { tlsEnabled: true, tags: { environment: 'production' } }

const result: GetDomainConfig200Response = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
[inline-code-end]
