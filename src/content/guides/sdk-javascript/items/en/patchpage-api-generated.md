## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPIPageData | UpdateAPIPageData | Yes |  |

## Response

Returns: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchPageAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'patchPage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-enterprises';
  const id: string = '9f8b7c6d-4a2e-11ec-8d3d-0242ac130003';
  const updateData: UpdateAPIPageData = { title: 'Q2 Financial Summary' }; // other fields optional
  const result: PatchPageAPIResponse = await patchPage(tenantId, id, updateData);
  console.log(result);
})();
[inline-code-end]
