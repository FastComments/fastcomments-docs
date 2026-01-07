[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Diese Route ermöglicht das Entfernen eines einzelnen SSO-Benutzers nach seiner ID.

Beachten Sie, dass das erneute Laden des Kommentar-Widgets mit einem Payload für diesen Benutzer den Benutzer einfach nahtlos neu erstellt.

Das Löschen der Kommentare des Benutzers ist über den `deleteComments`-Abfrageparameter möglich. Beachten Sie, dass wenn dies wahr ist:

1. Alle Kommentare des Benutzers werden live gelöscht.
2. Alle __untergeordneten__ (nun verwaisten) Kommentare werden basierend auf der zugehörigen Seitenkonfiguration jedes Kommentars gelöscht oder anonymisiert. Wenn beispielsweise der Thread-Löschmodus "anonymisieren" ist, bleiben Antworten erhalten und die Kommentare des Benutzers werden anonymisiert. Dies gilt nur, wenn `commentDeleteMode` `Remove` ist (der Standardwert).
3. Die `creditsCost` wird zu `2`.

### Anonymisierte Kommentare

Sie können die Kommentare des Benutzers behalten, aber einfach anonymisieren, indem Sie `commentDeleteMode=1` setzen.

Wenn die Kommentare des Benutzers anonymisiert werden, werden die folgenden Werte auf null gesetzt:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` und `isDeletedUser` wird auf `true` gesetzt.

Beim Rendern verwendet das Kommentar-Widget `DELETED_USER_PLACEHOLDER` (Standard: "[deleted]") für den Namen des Benutzers und `DELETED_CONTENT_PLACEHOLDER` für den Kommentar. Diese können über die Widget-Anpassungs-Benutzeroberfläche angepasst werden.

### Beispiele

[inline-code-attrs-start title = 'SSOUser Entfernen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Entfernen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'SSOUser Entfernen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
