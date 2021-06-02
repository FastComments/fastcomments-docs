[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; costPerPageLoad = 1; api-resource-header-end]

This route allows the creation of a single SSO user.

Trying to create two users with the same ID will result in an error.

[inline-code-attrs-start title = 'SSOUser Creation cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"email": "fordperfect@galaxy.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Creation Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Creation Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists';
    /** Included on failure. **/
    reason?: string;
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]
