## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-inc-78f9';
const includeDrafts: boolean | undefined = undefined;
const pageSize: number | undefined = 25;
const pagesResponse: GetPagesAPIResponse = await getPages(tenantId);
const pages: APIPage[] | undefined = (pagesResponse as any).pages;
const firstPageTitle: string | undefined = pages?.[0]?.title as string | undefined;
[inline-code-end]
