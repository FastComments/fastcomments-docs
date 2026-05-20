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
const tenantId: string = 'acme-enterprises-42';
const pageId: string = 'f4d2c8b1-77e3-4a9f';
async function removePage(tenantIdParam: string, idParam: string, purge?: boolean): Promise<DeletePageAPIResponse> {
  const response: DeletePageAPIResponse = await deletePage(tenantIdParam, idParam);
  return response;
}
const deleted: DeletePageAPIResponse = await removePage(tenantId, pageId, true);
[inline-code-end]
