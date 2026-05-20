## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'putDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_88a2b7';
const domainToUpdate: string = 'login.enterprise-acme.com';
const updateDomainConfigParams: UpdateDomainConfigParams = {
  enabled: true,
  ttlSeconds: 3600,
  redirectToHttps: true,
  aliases: ['login.acme-staging.com'], // optional parameter demonstrated
  metadata: { owner: 'platform-team', environment: 'production' }
};
const updatedConfig: GetDomainConfig200Response = await putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
[inline-code-end]
