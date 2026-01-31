## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const options: { includeDrafts?: boolean; pageSize?: number } = { includeDrafts: true, pageSize: 50 };
const pagesResponse: GetPagesAPIResponse = await getPages(tenantId, options);
[inline-code-end]
