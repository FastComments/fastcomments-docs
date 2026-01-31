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
const tenantId: string = 'tenant_acme_01';
const id: string = 'page_2026-01-site-performance';
const update: UpdateAPIPageData = {
  title: 'Site Performance Guide — January 2026',
  path: '/engineering/site-performance-2026',
  isEnabled: true,
  description: 'Revised benchmarks and measurement techniques for 2026.',
  metadata: { author: 'Jane Doe', updatedAt: '2026-01-06T09:30:00Z' } // optional fields demonstrated
};
const response: PatchPageAPIResponse = await patchPage(tenantId, id, update);
[inline-code-end]
