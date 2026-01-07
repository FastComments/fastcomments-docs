[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

This route provides the ability to update a single `TenantUser`.

Updating a `TenantUser` has the following restrictions:

- The `signUpDate` may not be in the future.
- The `locale` must be in the list of [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- The `username` must be unique across all of FastComments.com. If this is an issue, we suggest using SSO instead.
- The `email` must be unique across all of FastComments.com. If this is an issue, we suggest using SSO instead.
- You cannot update the `tenantId` of a user.

We can create a `TenantUser` as follows

[inline-code-attrs-start title = 'TenantUser Update cURL Example'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Update Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Update Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
