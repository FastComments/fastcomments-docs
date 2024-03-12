[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

This route provides the removal of a single SSO user by their id.

Note that loading the comment widget again with a payload for this user will simply recreate the user seamlessly.

Deleting the user's comments is possible via the `deleteComments` query parameter. Note that if this is true:

1. All the user's comments will be deleted live.
2. All __child__ (now orphan) comments will be deleted or anonymized based on each comment's associated page configuration. For example if thread deletion mode is "anonymize", then replies will remain, and the user's comments will be anonymized. This only applies when `commentDeleteMode` is `Remove` (the default value).
3. The `creditsCost` becomes `2`.

You can retain the user's comments but simply anonymize them by setting `commentDeleteMode=1`.

If the user's comments are anonymized then the following values are set to null:

    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

The comment name (`commenterName`) is set to the translated value of `COMMENT_DELETED_NAME`.

[inline-code-attrs-start title = 'SSOUser Removal cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Removal Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
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
