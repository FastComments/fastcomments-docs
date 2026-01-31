## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageByURLIdAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getPageByURLId Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-84a2';
const urlId: string = 'homepage-127b';
const response: GetPageByURLIdAPIResponse = await getPageByURLId(tenantId, urlId);
const page: APIPage | undefined = response?.page;
const title: string | undefined = page?.title;
console.log(title ?? 'Untitled page');
[inline-code-end]
