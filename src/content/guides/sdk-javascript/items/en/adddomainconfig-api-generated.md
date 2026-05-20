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
const tenantId: string = "tenant_7f3b2a1c9";
const addDomainConfigParams: AddDomainConfigParams = {
  domain: "payments.mybusiness.com",
  primary: true,
  validateDns: true
};
const result: AddDomainConfig200Response = await addDomainConfig(tenantId, addDomainConfigParams);
[inline-code-end]
