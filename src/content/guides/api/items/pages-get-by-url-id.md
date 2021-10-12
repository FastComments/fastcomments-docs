[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages/by-url-id'; costPerPageLoad = 1; api-resource-header-end]

Individual pages can be fetched by their corresponding `urlId`. This can be useful for looking up page titles or comment counts. 

[inline-code-attrs-start title = 'Page by URL ID cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/by-url-id?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page By URL ID Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string;
    API_KEY: string;
    urlId: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'Page by URL ID Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id';
    /** Included on failure. **/
    reason?: string;
    page?: Page[] | null;
}
[inline-code-end]

#### Helpful Tip

Remember to URI Encode values like the `urlId`.
