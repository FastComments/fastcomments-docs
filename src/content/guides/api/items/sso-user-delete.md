[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

This route provides the removal of a single SSO user by their id.

Note that loading the comment widget again with a payload for this user will simply recreate the user seamlessly.

Deleting the user's comments is possible via the `deleteComments` query parameter. Note that if this is true:

1. All the user's comments will be deleted live.
2. All __child__ (now orphan) comments will also be deleted.
3. The `creditsCost` becomes `2`.

[inline-code-attrs-start title = 'SSOUser Removal cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Removal Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Removal Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the removed user on success.
}
[inline-code-end]
