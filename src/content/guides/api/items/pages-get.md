[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; costPerPageLoad = 1000; api-resource-header-end]

You can currently only fetch all pages associated with your account. If you'd like more fine-grained searching, [reach out to us](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Pages cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Pages Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'Pages Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key';
    /** Included on failure. **/
    reason?: string;
    pages: Page[]
}
[inline-code-end]

#### Helpful Tip

The `Comment` API requires a `urlId`. You can call the `Pages` API first, to see what the `urlId` values available to you
look like.
