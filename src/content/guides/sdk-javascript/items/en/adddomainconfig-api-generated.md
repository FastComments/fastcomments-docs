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
const tenantId: string = "org-3fa9d8b2";
const addDomainConfigParams: AddDomainConfigParams = {
  domainName: "login.acme-corp.com",
  isPrimary: true,
  verification: { method: "dns", token: "verify-9c3b" },
  dnsRecords: [{ type: "TXT", name: "_acme", value: "v=spf1 include:_spf.example.com ~all" }] // optional parameter included
};
const result: AddDomainConfig200Response = await addDomainConfig(tenantId, addDomainConfigParams);
[inline-code-end]
