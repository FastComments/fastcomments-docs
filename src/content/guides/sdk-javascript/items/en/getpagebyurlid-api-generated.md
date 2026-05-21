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
interface APIPage { id: string; title: string; content?: string; published: boolean }
interface GetPageByURLIdAPIResponse { page: APIPage; retrievedAt?: string }

const tenantId: string = 'acme-enterprises';
const urlId: string = '6f1e2d3c-4b5a-6789-abcd-1234567890ef';

const result: GetPageByURLIdAPIResponse = await getPageByURLId(tenantId, urlId);

const pageTitle: string = result.page.title;
const contentPreview: string | undefined = result.page.content?.slice(0, 120);
[inline-code-end]
