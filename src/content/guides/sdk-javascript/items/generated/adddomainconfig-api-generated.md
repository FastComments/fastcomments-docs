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
const tenantId: string = "tenant_7f3c9b2a9e1";
const addDomainConfigParams: AddDomainConfigParams = {
  domain: "comments.production.example.com",
  enableHttps: true,
  allowedOrigins: ["https://example.com", "https://dashboard.example.com"],
  cname: undefined, // optional parameter explicitly omitted
};
const result: AddDomainConfig200Response = await addDomainConfig(tenantId, addDomainConfigParams);
[inline-code-end]
