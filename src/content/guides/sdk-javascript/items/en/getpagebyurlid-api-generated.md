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
const tenantId: string = 'acme-corp-842';
const urlId: string = 'home-landing';
const pageResponse: GetPageByURLIdAPIResponse = await getPageByURLId(tenantId, urlId);
const page: APIPage | undefined = (pageResponse as unknown as { page?: APIPage }).page;
[inline-code-end]
