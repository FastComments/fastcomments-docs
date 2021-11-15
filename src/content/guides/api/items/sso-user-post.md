[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

This route provides the creation of a single SSO user.

Trying to create two users with the same ID will result in an error.

[inline-code-attrs-start title = 'SSOUser Creation cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

In this example we specify `groupIds` for access control, but this is optional.

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

#### Integration Note

Data passed by the API can be overridden simply by passing a different SSO User HMAC payload. For example, if
you set a username via the API, but then pass a different one via the SSO flow on page load, we will automatically update
their username.

We will not update user parameters in this flow unless you explicitly specify them or set them to null (not undefined).
