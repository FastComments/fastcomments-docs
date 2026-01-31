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
const tenantId: string = 'acme-enterprises-7f9b4c2e';
const id: string = 'page-home-2026-01-09';
const result: DeletePageAPIResponse = await deletePage(tenantId, id);
[inline-code-end]
