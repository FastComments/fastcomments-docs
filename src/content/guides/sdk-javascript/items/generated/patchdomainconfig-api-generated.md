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
const tenantId: string = "tenant_4f2b1a9e-6c3d-4e5f-b1a2-abcdef123456";
const domainToUpdate: string = "comments.acme-products.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  primary: true,
  enableSsl: true,
  sslCertificateId: "ssl_cert_72f4b1",
  allowedOrigins: ["https://www.acme.com", "https://shop.acme.com"],
  forceHttpsRedirect: undefined
};
const updatedConfig: GetDomainConfig200Response = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]
