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
const tenantId: string = 'tenant_6a9f8b2c';
const params: AddDomainConfigParams = {
  domain: 'comments.myproduct.io',
  allowedOrigins: ['https://www.myproduct.io', 'https://app.myproduct.io'],
  enforceHttps: true, // optional parameter
  description: 'Comments subdomain for product site'
};
const response: AddDomainConfig200Response = await addDomainConfig(tenantId, params);
[inline-code-end]
