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
const tenantId: string = "acme-enterprises-tenant-42";
const urlId: string = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
const includeArchived: boolean | undefined = undefined; // optional flag example (not required by getPageByURLId)
const pageResponse: GetPageByURLIdAPIResponse = await getPageByURLId(tenantId, urlId);
const page: APIPage | undefined = (pageResponse as any)?.page;
const pageTitle: string | undefined = (page as any)?.title;
[inline-code-end]
