## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | Yes |  |

## Response

Returns: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddPageAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'addPage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-9843';
const createAPIPageData: CreateAPIPageData = {
  url: '/blog/2026/typescript-addpage-example',
  title: 'Integrating FastComments addPage',
  description: 'Adding a new documentation page for comments',
  allowComments: true, // optional flag demonstrating optional parameters
  tags: ['docs', 'typescript']
};
const result: AddPageAPIResponse = await addPage(tenantId, createAPIPageData);
const createdPage: APIPage | undefined = result.page;
[inline-code-end]
