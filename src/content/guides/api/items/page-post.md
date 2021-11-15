[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

This API endpoint provides the ability to create pages.

A common use cases is access control.

Notes:

- If you've commented on a comment thread, or called the API to create a `Comment`, you've already created a `Page` object! You can try fetching it via
  the `/by-url-id` `Page` route, passing in the same `urlId` passed to the comment widget.
- The `Page` structure contains some **calculated** values.
  Currently, these are `commentCount` and `rootCommentCount`.
  They are populated automatically and cannot be set by the API. Attempting to do so will cause the API to return an error.

[inline-code-attrs-start title = 'Page POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page POST Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Included on failure. **/
    reason?: string;
    /** The created page. **/
    page?: Page;
}
[inline-code-end]
