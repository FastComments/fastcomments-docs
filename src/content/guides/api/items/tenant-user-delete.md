[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

This route provides the removal of a `TenantUser` by id.

Deleting the user's comments is possible via the `deleteComments` query parameter. Note that if this is true:

1. All the user's comments will be deleted live.
2. All __child__ (now orphan) comments will be deleted or anonymized based on each comment's associated page configuration. For example if thread deletion mode is "anonymize", then replies will remain, and the user's comments will be anonymized. This only applies when `commentDeleteMode` is `Remove` (the default value).
3. The `creditsCost` becomes `2`.

### Anonymized Comments

You can retain the user's comments but simply anonymize them by setting `commentDeleteMode=1`.

If the user's comments are anonymized then the following values are set to null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` is set to `true`.

When rendering, the comment widget will use `DELETED_PLACEHOLDER` (default: "[deleted]") for the user's name. These can be customized via the Widget Customization UI.

### Examples

[inline-code-attrs-start title = 'TenantUser Removal cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Removal Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Removal Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
