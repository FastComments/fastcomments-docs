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
const tenantId: string = 'acme-enterprises';
const id: string = 'pg_6f4c2b1a';
const updateAPIPageData: UpdateAPIPageData & { notifyTeam?: boolean } = {
  title: 'Q3 Product Roadmap',
  content: '<p>Finalize feature set and schedule beta releases.</p>',
  notifyTeam: true
};
const result: PatchPageAPIResponse = await patchPage(tenantId, id, updateAPIPageData);
[inline-code-end]
