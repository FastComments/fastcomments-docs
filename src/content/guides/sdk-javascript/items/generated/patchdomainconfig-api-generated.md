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
const tenantId: string = 'tenant_acme_42';
const domainToUpdate: string = 'comments.acme-app.com';
const patchDomainConfigParams: PatchDomainConfigParams = {
  enableHttps: true,
  allowedOrigins: ['https://www.acme-app.com', 'https://app.acme-app.com'], // optional list
  redirectToPrimaryDomain: false // optional toggle
} as PatchDomainConfigParams;
const updatedConfig: GetDomainConfig200Response = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]
