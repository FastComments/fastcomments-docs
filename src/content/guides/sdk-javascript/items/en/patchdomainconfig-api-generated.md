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
const tenantId: string = "acme-tenant-001";
const domainToUpdate: string = "billing.acme-corp.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  ssl: { enabled: true, certificateId: "cert-prod-12345" }, // optional fields included
  redirect: { from: "http://billing.acme-corp.com", to: "https://billing.acme-corp.com", permanent: true }
};
const updatedConfig: GetDomainConfig200Response = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]
