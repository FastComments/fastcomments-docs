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
const tenantId: string = 'tenant_8d3f2b4a9c';
const domain: string = 'comments.myproduct.io';
const response: DeleteDomainConfig200Response = await deleteDomainConfig(tenantId, domain);
// Optional post-delete handler (demonstrates optional parameter usage)
const onDeleted?: (res: DeleteDomainConfig200Response) => void = (res) => { /* notify admin, refresh UI */ };
onDeleted?.(response);
[inline-code-end]
