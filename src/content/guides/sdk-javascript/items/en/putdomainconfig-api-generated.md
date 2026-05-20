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
const tenantId: string = "tenant_9b3f21";
const domainToUpdate: string = "payments.acme-corp.com";
const updateDomainConfigParams: UpdateDomainConfigParams = {
  enableHttps: true,
  certificateId: "cert_7a2d55", // optional parameter supplied
  ttl: 3600,
  redirects: [{ from: "/legacy-checkout", to: "/checkout", statusCode: 301 }]
};
const result: GetDomainConfig200Response = await putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
[inline-code-end]
