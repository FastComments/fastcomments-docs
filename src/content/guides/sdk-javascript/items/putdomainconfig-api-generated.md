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
const tenantId: string = "tenant_93b4e2";
const domainToUpdate: string = "comments.prod-fastcomments.com";
const updateDomainConfigParams: UpdateDomainConfigParams = {
  allowSubdomains: true,
  enforceHttps: true,
  trustedOrigins: ["https://app.example.com"],
  note: "Enable subdomains, force HTTPS, and allow app origin"
};
const result: GetDomainConfig200Response = await putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
[inline-code-end]
