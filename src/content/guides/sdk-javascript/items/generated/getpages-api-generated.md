## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8a7b3d2f';
  const result: GetPagesAPIResponse = await getPages(tenantId);
  const pages: APIPage[] = result.pages ?? [];
  const firstPageId: string | undefined = pages[0]?.id;
  console.log(`Tenant ${tenantId} has ${pages.length} pages; first page id: ${firstPageId}`);
})();
[inline-code-end]
