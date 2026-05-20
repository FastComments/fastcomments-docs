## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'b7c9f1e2-3a8d-4f6b-9c1a-2e7d4b5f6a9c';
const options: { limit?: number; includeDrafts?: boolean } = { limit: 25 };
const pages: GetPagesAPIResponse = await getPages(tenantId, options);
[inline-code-end]
