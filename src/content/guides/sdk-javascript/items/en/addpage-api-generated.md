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
const tenantId: string = 'acme-enterprises-01';
const newPageData: CreateAPIPageData = {
  title: 'Customer Support',
  slug: '/support',
  contentHtml: '<p>Contact support at support@acme.com or call +1-555-0100.</p>',
  authorId: 'user-782',
  tags: ['support', 'customer-care'],
  publishAt: undefined // optional field demonstrated
};
const response: AddPageAPIResponse = await addPage(tenantId, newPageData);
[inline-code-end]
