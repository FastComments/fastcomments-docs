[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

This route provides the removal of a single SSO user by their id.

Note that loading the comment widget again with a payload for this user will simply recreate the user seamlessly.

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

`isDeleted` and `isDeletedUser` is set to `true`.

When rendering, the comment widget will use `DELETED_USER_PLACEHOLDER` (privzeto: "[deleted]") for the user's name and `DELETED_CONTENT_PLACEHOLDER` for the comment. These can be customized via the Widget Customization UI.

### Examples

[inline-code-attrs-start title = 'Primer cURL zahteve za odstranitev SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za odstranitev SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // privzeto
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Nastavite to na 'true', če želite tudi izbrisati komentarje uporabnika. To bo podvojilo strošek kreditov. **/
    deleteComments?: 'true' | 'false'
    /** Nastavite to po želji, da določite, kako ravnati s komentarji uporabnika. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za odstranitev SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Vključeno ob napaki. **/
    reason?: string
    user?: SSOUser; // Ob uspehu vrnemo odstranjenega uporabnika.
}
[inline-code-end]