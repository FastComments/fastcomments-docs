## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3f8d9b2a";
const options: { page?: number; pageSize?: number } = { page: 1, pageSize: 25 };
const pagesResponse: GetPagesAPIResponse = await getPages(tenantId, options);
[inline-code-end]
