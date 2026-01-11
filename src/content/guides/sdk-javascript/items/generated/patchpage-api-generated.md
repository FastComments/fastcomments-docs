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
const tenantId: string = 'acme-tenant-01';
const pageId: string = 'page_4f7a2b9d';
const updateData: UpdateAPIPageData = {
  title: 'About Acme Corporation',
  slug: '/about',
  isPublished: true,
  tags: ['company', 'about'], // optional array of tags
  metadata: { description: 'Company history and mission statement.' }, // optional nested object
  lastEditedBy: 'j.smith@acme.com' // optional auditor field
};
const response: PatchPageAPIResponse = await patchPage(tenantId, pageId, updateData);
[inline-code-end]
