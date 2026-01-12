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
const tenantId: string = 'tenant_7f3b2a4c';
const pageId: string = 'page_9c1d8f7';
const updateAPIPageData: UpdateAPIPageData = {
  title: 'How to use FastComments with TypeScript',
  path: '/guides/fastcomments-typescript',
  isPublished: true,
  metadata: { author: 'Alex Martinez', tags: ['comments', 'typescript'] } // optional field included
};
const result: PatchPageAPIResponse = await patchPage(tenantId, pageId, updateAPIPageData);
[inline-code-end]
