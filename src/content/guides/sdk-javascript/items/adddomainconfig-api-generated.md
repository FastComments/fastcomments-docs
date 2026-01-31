## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| addDomainConfigParams | AddDomainConfigParams | Yes |  |

## Response

Returns: [`AddDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'addDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7a1c4d2e';
const addDomainConfigParams: AddDomainConfigParams = {
  domain: 'comments.mycompany.com',
  allowSubdomains: true,
  redirectToHttps: true,
  cnameTarget: 'fc-9b8c7d.fastcomments.com',
  enabled: true,
  ttl: 3600 // optional parameter: DNS TTL in seconds
};
const result: AddDomainConfig200Response = await addDomainConfig(tenantId, addDomainConfigParams);
[inline-code-end]
