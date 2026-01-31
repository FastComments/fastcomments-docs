## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'getDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-prod-92f7';
  const subdomain?: string = 'comments';
  const domain: string = subdomain ? `${subdomain}.acme-corp.com` : 'acme-corp.com';
  const config: GetDomainConfig200Response = await getDomainConfig(tenantId, domain);
  console.log(config);
})();
[inline-code-end]
