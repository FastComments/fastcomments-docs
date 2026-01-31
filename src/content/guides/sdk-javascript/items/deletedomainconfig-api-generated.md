## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b2f4c1a';
const domain: string = 'comments.acme-corp.com';
const result: DeleteDomainConfig200Response = await deleteDomainConfig(tenantId, domain);
console.log(result);
[inline-code-end]
