## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePageAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'deletePage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7c1ab2ef';
const id: string = 'b5f9c3d0-12ab-4e6f-9a2c-3d4b5a6e7f8a';
const response: DeletePageAPIResponse = await deletePage(tenantId, id);
[inline-code-end]
