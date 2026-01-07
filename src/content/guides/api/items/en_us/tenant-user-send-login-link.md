[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

This route provides the ability to send a login link to a single `TenantUser`.

Useful when batch creating users and not having to instruct them on how to login to FastComments.com. This will just send them a "magic link" to login that
expires after `30 days`.

The following restrictions exist to send a login link to a `TenantUser`:
- The `TenantUser` must already exist.
- You must have access to manage the `Tenant` the `TenantUser` belongs to.

We can send a login link to a `TenantUser` as follows:

[inline-code-attrs-start title = 'TenantUser Login Link cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

This will send an email like `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'TenantUser Login Link Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Login Link Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
