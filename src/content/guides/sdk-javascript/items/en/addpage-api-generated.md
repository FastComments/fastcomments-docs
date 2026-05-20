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
const tenantId: string = 'tenant_9b3f2c1a';
const createData: CreateAPIPageData = {
  title: 'Q3 Product Roadmap',
  slug: 'q3-product-roadmap',
  content: '<h1>Goals</h1><p>Focus on performance and reliability improvements.</p>',
  authorId: 'user_84c2',
  published: false,            // optional flag demonstrated
  tags: ['product', 'roadmap'] // optional metadata demonstrated
};
const result: AddPageAPIResponse = await addPage(tenantId, createData);
[inline-code-end]
